extern crate winapi;
extern crate serde;
extern crate serde_json;

use std::fs;
use std::process;
use std::io::{self, Write};
use std::time::Duration;
use ravenbot::commands::only_walk_path_walker;
use ravenbot::utils::env::{Food, Timer};
use tokio::time::sleep;
use winapi::um::winuser::{GetAsyncKeyState, VK_F1, VK_F12};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use winapi::shared::minwindef::{LPARAM, BOOL, DWORD};
use winapi::{
    um::winuser::{EnumWindows, GetWindowThreadProcessId, IsWindowVisible},
    shared::windef::HWND,
};
use std::ptr;
use std::thread;

use ravenbot::utils::address::get_base_address;
use ravenbot::utils::env::{Config, Hunt, Walk, Skills, Combat, Foods, CooldownManager};
use ravenbot::utils::inputs::press_skill;
use ravenbot::commands::hunting_instance;
use ravenbot::commands::only_combat_instance;
use ravenbot::commands::hunting_path_walker;
use ravenbot::checks::get_coord;
use ravenbot::checks::check_hwid;

use tokio::{time::interval, task};

use chrono::{Local, Timelike};
use log::{info, error};
use env_logger::Env;

struct WindowInfo {
    game_p_id: DWORD,
    hwnd: HWND,
}

// Implementaçao de window global para não precisar passar entre threads
static mut WINDOW_HANDLE: HWND = std::ptr::null_mut();

fn get_window_handle() -> Result<HWND, io::Error> {

    let (_base_address, process_id) = match get_base_address() {
        Some(data) => data,
        None => {
            error!("Erro ao encontrar o endereço base do módulo");
            return Err(io::Error::new(io::ErrorKind::Other, "Erro ao encontrar o endereço base do módulo"));
        }
    };

    let mut window_info = WindowInfo {
        game_p_id: process_id,
        hwnd: ptr::null_mut(),
    };

    unsafe {
        EnumWindows(Some(enum_windows_callback), &mut window_info as *mut _ as LPARAM);
    }
    
    if window_info.hwnd.is_null() {
        error!("Janela não encontrada");
        return Err(io::Error::new(io::ErrorKind::Other, "Janela não encontrada"));
    } else { 
        unsafe { WINDOW_HANDLE = window_info.hwnd }
        Ok(unsafe { WINDOW_HANDLE })
    }
}

extern "system" fn enum_windows_callback(window: HWND, param: LPARAM) -> BOOL {
    let window_info = unsafe { &mut *(param as *mut WindowInfo) };

    let mut p_id: DWORD = 0;
    unsafe {
        GetWindowThreadProcessId(window, &mut p_id);
    }
    if p_id == window_info.game_p_id && unsafe { IsWindowVisible(window) } != 0 {
        window_info.hwnd = window;
        return 0; // Retorna falso para parar a enumeração
    }
    1 // Retorna verdadeiro para continuar a enumeração
}

async fn run_timer_for_foods(food: Food, running: Arc<AtomicBool>) {
    let _hwnd = match get_window_handle() {
        Ok(hwnd) => hwnd,
        Err(err) => {
            error!("Failed to get window handle: {}", err);
            running.store(false, Ordering::SeqCst);
            return;
        }
    };

    let mut interval = interval(Duration::from_secs(food.timer * 60)); // X minutos baseado no config

    while running.load(Ordering::SeqCst) {
        interval.tick().await;
        use_foods(&food);
    }
}

pub fn use_foods(food: &Food){
    let brt = chrono::FixedOffset::west_opt(3 * 3600).unwrap(); // Horário de Brasília (UTC-3)
    let current_time = Local::now().with_timezone(&brt);
    info!("Using {} at {:02}:{:02}:{:02} BRT..", food.name, current_time.hour(), current_time.minute(), current_time.second());
    press_skill(unsafe { WINDOW_HANDLE }, &food.hotkey);
    thread::sleep(Duration::from_millis(100));
}

async fn run_timer_general(running: Arc<AtomicBool>, timer: Timer) {
    let _hwnd = match get_window_handle() {
        Ok(hwnd) => hwnd,
        Err(err) => {
            error!("Failed to get window handle: {}", err);
            running.store(false, Ordering::SeqCst);
            return;
        }
    };

    let mut count = 0; // Move a variável count para fora do loop para mantê-la entre as iterações

    let mut interval = interval(Duration::from_secs(60 * 60));

    while running.load(Ordering::SeqCst) {
        interval.tick().await;
        info!("You are playing for {} hours.", count);
        count += 1; // Incrementa o contador em cada iteração do loop
        if timer.flag && timer.hours < count {
            info!("Finishing process according to {} hours on config.", timer.hours);
            process::exit(0); // Encerra o programa com um código de status 0
        }
    }
}

fn read_config() -> Config {

    let combat_contents = match fs::read_to_string("config/combat.json") {
        Ok(contents) => contents,
        Err(err) => {
            eprintln!("Erro ao ler o arquivo combat.json: {}", err);
            process::exit(1); // Encerra o programa com um código de status 1;
        }
    };
    let config_combat: Combat = match serde_json::from_str(&combat_contents) {
        Ok(value) => value,
        Err(err) => {
            eprintln!("Erro ao desserializar o arquivo combat.json: {}", err);
            process::exit(1); // Encerra o programa com um código de status 1;
        }
    };

    let skills_contents = match fs::read_to_string("config/skills.json") {
        Ok(contents) => contents,
        Err(err) => {
            eprintln!("Erro ao ler o arquivo combat.json: {}", err);
            process::exit(1); // Encerra o programa com um código de status 1;
        }
    };
    let config_skills: Skills = match serde_json::from_str(&skills_contents) {
        Ok(value) => value,
        Err(err) => {
            eprintln!("Erro ao desserializar o arquivo skills.json: {}", err);
            process::exit(1); // Encerra o programa com um código de status 1;
        }
    };

    let hunts_contents = match fs::read_to_string("config/hunts.json") {
        Ok(contents) => contents,
        Err(err) => {
            eprintln!("Erro ao ler o arquivo combat.json: {}", err);
            process::exit(1); // Encerra o programa com um código de status 1;
        }
    };
    let config_hunts: Vec<Hunt> = match serde_json::from_str(&hunts_contents) {
        Ok(value) => value,
        Err(err) => {
            eprintln!("Erro ao desserializar o arquivo hunts.json: {}", err);
            process::exit(1); // Encerra o programa com um código de status 1;
        }
    };

    let foods_contents = match fs::read_to_string("config/foods.json") {
        Ok(contents) => contents,
        Err(err) => {
            eprintln!("Erro ao ler o arquivo combat.json: {}", err);
            process::exit(1); // Encerra o programa com um código de status 1;
        }
    };
    let config_foods: Foods = match serde_json::from_str(&foods_contents) {
        Ok(value) => value,
        Err(err) => {
            eprintln!("Erro ao desserializar o arquivo foods.json: {}", err);
            process::exit(1); // Encerra o programa com um código de status 1;
        }
    };

    let walks_contents = match fs::read_to_string("config/walks.json") {
        Ok(contents) => contents,
        Err(err) => {
            eprintln!("Erro ao ler o arquivo combat.json: {}", err);
            process::exit(1); // Encerra o programa com um código de status 1;
        }
    };
    let config_walks: Vec<Walk> = match serde_json::from_str(&walks_contents) {
        Ok(value) => value,
        Err(err) => {
            eprintln!("Erro ao desserializar o arquivo walks.json: {}", err);
            process::exit(1); // Encerra o programa com um código de status 1;
        }
    };

    let timer_contents = match fs::read_to_string("config/timer.json") {
        Ok(contents) => contents,
        Err(err) => {
            eprintln!("Erro ao ler o arquivo combat.json: {}", err);
            process::exit(1); // Encerra o programa com um código de status 1;
        }
    };
    let config_timer: Timer = match serde_json::from_str(&timer_contents) {
        Ok(value) => value,
        Err(err) => {
            eprintln!("Erro ao desserializar o arquivo timer.json: {}", err);
            process::exit(1); // Encerra o programa com um código de status 1;
        }
    };
    
    let config_contents = Config {
        hunts: config_hunts,
        combat: config_combat,
        skills: config_skills,
        foods: config_foods,
        walks: config_walks,
        timer: config_timer
    };

    return config_contents;
}

fn main_menu() -> io::Result<()> {

    let config = read_config();

    println!("Selecione uma opção:");
    println!("1: Create Hunting Coordinates");
    println!("2: Hunting");
    println!("3: Only Combat (Manual Walk)");
    println!("4: Create Walk Coordinates");
    println!("5: Only Walk (No Combat)");

    let mut choice = String::new();
    io::stdin().read_line(&mut choice)?;

    match choice.trim() {
        "1" => create_hunting_coordinates(),
        "2" => hunting(config),
        "3" => only_combat(config),
        "4" => create_walk_coordinates(),
        "5" => only_walk(config),
        _ => {
            println!("Opção inválida, por favor, tente novamente.");
            main_menu()
        }
    }

}

fn create_hunting_coordinates() -> io::Result<()> {
    let running = Arc::new(AtomicBool::new(true));
    let r = running.clone();

    ctrlc::set_handler(move || {
        r.store(false, Ordering::SeqCst);
    }).expect("Erro ao definir o manipulador de Ctrl-C");

    println!("A sua hunt vai ter escadas? ");
    println!("1: Sim ");
    println!("0: Não ");
    let mut stairs_choice = String::new();
    let mut stairs_bool = false;
    io::stdin().read_line(&mut stairs_choice)?;
    match stairs_choice.trim() {
        "1" => { 
            stairs_bool = true; 
        },
        "0" => {
            stairs_bool = false;
        }
        _ => {
            println!("Opção inválida, por favor, tente novamente.");
            let _ = create_hunting_coordinates();
        }
    }

    print!("Digite o nome da sua hunt: ");
    let mut nome = String::new();
    io::stdout().flush().unwrap();

    io::stdin().read_line(&mut nome).expect("Falha ao ler a entrada");
    let nome = nome.trim().to_string();

    let file_path = "config/hunts.json";
    let mut config = read_config();

    // Verifica se já existe uma caçada com o nome fornecido, se não, adiciona uma nova
    let hunt_index = config.hunts.iter().position(|h| h.name == nome);
    let _hunt = match hunt_index {
        Some(index) => &mut config.hunts[index],
        None => {
            config.hunts.push(Hunt { name: nome.clone(), route: Vec::new(), stairs: stairs_bool });
            config.hunts.last_mut().unwrap()
        },
    };

    info!("Pressione 'F1' para salvar a coordenada da posição atual.");
    info!("Pressione 'F12' para finalizar o processo.");
    while running.load(Ordering::SeqCst) {
        unsafe {
            if GetAsyncKeyState(VK_F1 as i32) as u16 & 0x8000 != 0 {
                let current_value = get_coord(); // Supondo que isso retorna um [i32; 3]
                info!("Coordenada adicionada: {:?}", current_value);
    
                // Atualize a rota diretamente sem manter um empréstimo mutável longo
                if let Some(_hunt) = config.hunts.iter_mut().find(|h| h.name == nome) {
                    _hunt.route.push(current_value);
                } else {
                    config.hunts.push(Hunt { name: nome.clone(), route: vec![current_value], stairs: stairs_bool });
                }
    
                // Agora que as modificações foram feitas, podemos serializar
                let json_string = serde_json::to_string_pretty(&config.hunts).expect("Falha ao serializar JSON");
                fs::write(file_path, json_string.as_bytes()).expect("Falha ao escrever no arquivo");
    
                thread::sleep(Duration::from_secs(1)); // Evita capturas duplicadas
            }
            if GetAsyncKeyState(VK_F12 as i32) as u16 & 0x8000 != 0 {
                info!("Finalizando...");
                process::exit(0); // Encerra o programa com um código de status 0
            }
        }
    }
    
    Ok(())
}

fn create_walk_coordinates() -> io::Result<()> {
    let running = Arc::new(AtomicBool::new(true));
    let r = running.clone();

    ctrlc::set_handler(move || {
        r.store(false, Ordering::SeqCst);
    }).expect("Erro ao definir o manipulador de Ctrl-C");

    print!("Digite o nome da sua walk: ");
    io::stdout().flush().unwrap();
    let mut nome = String::new();
    io::stdin().read_line(&mut nome).expect("Falha ao ler a entrada");
    let nome = nome.trim().to_string();

    let file_path = "config/walks.json";
    let mut config = read_config();

    // Verifica se já existe uma walk com o nome fornecido, se não, adiciona uma nova
    let walk_index = config.walks.iter().position(|w| w.name == nome);
    let _walk = match walk_index {
        Some(index) => &mut config.walks[index],
        None => {
            config.walks.push(Walk { name: nome.clone(), route: Vec::new() });
            config.walks.last_mut().unwrap()
        },
    };

    while running.load(Ordering::SeqCst) {
        unsafe {
            if GetAsyncKeyState(VK_F1 as i32) as u16 & 0x8000 != 0 {
                let current_value = get_coord(); // Supondo que isso retorna um [i32; 3]
                info!("Coordenada adicionada: {:?}", current_value);
    
                // Atualize a rota diretamente sem manter um empréstimo mutável longo
                if let Some(_walk) = config.walks.iter_mut().find(|w| w.name == nome) {
                    _walk.route.push(current_value);
                } else {
                    config.walks.push(Walk { name: nome.clone(), route: vec![current_value] });
                }
    
                // Agora que as modificações foram feitas, podemos serializar
                let json_string = serde_json::to_string_pretty(&config.walks).expect("Falha ao serializar JSON");
                fs::write(file_path, json_string.as_bytes()).expect("Falha ao escrever no arquivo");
    
                thread::sleep(Duration::from_secs(1)); // Evita capturas duplicadas
            }
        }
    }
    
    Ok(())
}

fn choose_hunt(hunts: &[Hunt]) -> Option<usize> {
    println!("Escolha uma caçada:");
    for (index, hunt) in hunts.iter().enumerate() {
        println!("{}: {}", index + 1, hunt.name);
    }
    println!("Digite o número da caçada que deseja escolher:");

    let mut choice = String::new();
    if std::io::stdin().read_line(&mut choice).is_ok() {
        match choice.trim().parse::<usize>() {
            Ok(num) if num > 0 && num <= hunts.len() => Some(num - 1),
            _ => None,
        }
    } else {
        None
    }
}

fn choose_walk(walks: &[Walk]) -> Option<usize> {
    println!("Escolha uma walk:");
    for (index, walk) in walks.iter().enumerate() {
        println!("{}: {}", index + 1, walk.name);
    }
    println!("Digite o número da walk que deseja escolher:");

    let mut choice = String::new();
    if std::io::stdin().read_line(&mut choice).is_ok() {
        match choice.trim().parse::<usize>() {
            Ok(num) if num > 0 && num <= walks.len() => Some(num - 1),
            _ => None,
        }
    } else {
        None
    }
}

#[tokio::main]
async fn hunting(config: Config) -> io::Result<()> {

    let hunt_choice = choose_hunt(&config.hunts).expect("Escolha inválida de caçada.");
    let selected_hunt = config.hunts[hunt_choice].clone();
    let drink = config.combat.drink.clone();
    let hp_regen_passive = config.combat.hp_regen_passive.clone();
    let hp_to_continue = config.combat.hp_to_continue.clone();
    let mana_regen_passive = config.combat.mana_regen_passive.clone();
    let mana_to_continue = config.combat.mana_to_continue.clone();
    let hp_to_defense_light = config.combat.hp_to_defense_light.clone();
    let hp_to_defense_full = config.combat.hp_to_defense_full.clone();
    let combat_basic = config.skills.basic.clone();
    let combat_start = config.skills.start.clone();
    let combat_combo = config.skills.combo.clone();
    let combat_defense_light = config.skills.defense_light.clone();
    let combat_defense_full = config.skills.defense_full.clone();
    let global_cd = config.combat.global_cd.clone();

    let skills = config.skills.clone();
    let mut cooldown_manager = cooldown_manager_construct(&skills);

    let running = Arc::new(AtomicBool::new(true));
    let r = running.clone();

    ctrlc::set_handler(move || {
        r.store(false, Ordering::SeqCst);
    }).expect("Erro ao definir o manipulador de Ctrl-C");

    let timer_task = task::spawn(run_timer_general(running.clone(), config.timer));
    // Aguarde um curto período de tempo antes de iniciar a próxima tarefa
    sleep(Duration::from_millis(100)).await;
    let status_food_task = task::spawn(run_timer_for_foods(config.foods.status.clone(), running.clone()));
    // Aguarde um curto período de tempo antes de iniciar a próxima tarefa
    sleep(Duration::from_millis(100)).await;
    let attack_power_task = task::spawn(run_timer_for_foods(config.foods.attack_power.clone(), running.clone()));
    // Aguarde um curto período de tempo antes de iniciar a próxima tarefa
    sleep(Duration::from_millis(100)).await;
    let hp_mana_regen_food_task = task::spawn(run_timer_for_foods(config.foods.hp_mana_regen.clone(), running.clone()));
    // Aguarde um curto período de tempo antes de iniciar a próxima tarefa
    sleep(Duration::from_millis(100)).await;
    let hunting_task = task::spawn(async move {
        while running.load(Ordering::SeqCst) {
            for path in selected_hunt.route.iter() {
                hunting_instance(unsafe { WINDOW_HANDLE }, &hp_regen_passive, &mana_regen_passive, &hp_to_defense_light, &hp_to_defense_full, &combat_defense_light, &combat_defense_full, &combat_start, &combat_combo, &combat_basic, global_cd, &drink, &selected_hunt, &hp_to_continue, &mana_to_continue, &mut cooldown_manager);
                info!("Going to: {:?}", path);
                hunting_path_walker(unsafe { WINDOW_HANDLE }, *path, &hp_regen_passive, &mana_regen_passive, &hp_to_defense_light, &hp_to_defense_full, &combat_basic, &combat_start, &combat_combo, &combat_defense_light, &combat_defense_full, global_cd, &drink, &selected_hunt, &hp_to_continue, &mana_to_continue, &mut cooldown_manager);
            }
        }
    });

    let _ = timer_task.await?;
    let _ = status_food_task.await?;
    let _ = attack_power_task.await?;
    let _ = hp_mana_regen_food_task.await?;
    let _ = hunting_task.await?;
    
    Ok(())
}

#[tokio::main]
async fn only_walk(config: Config) -> io::Result<()> {
    let walk_choice = choose_walk(&config.walks).expect("Escolha inválida de walk.");
    let selected_walk = config.walks[walk_choice].clone();

    let running = Arc::new(AtomicBool::new(true));
    let r = running.clone();

    ctrlc::set_handler(move || {
        r.store(false, Ordering::SeqCst);
    }).expect("Erro ao definir o manipulador de Ctrl-C");

    let timer_task = task::spawn(run_timer_general(running.clone(), config.timer));
    let walking_task = task::spawn(async move {
        while running.load(Ordering::SeqCst) {
            for (index, path) in selected_walk.route.iter().enumerate() {
                info!("Going to: {:?}", path);
                only_walk_path_walker(unsafe { WINDOW_HANDLE }, *path);

                // Verifica se é o último elemento
                if index == selected_walk.route.len() - 1 {
                    info!("Reached the end of the walk!");
                    // Faça o que precisa ser feito quando o loop chega ao final do array
                    break;
                }
            }
            break;
        }
        process::exit(0); // Encerra o programa com um código de status 0
    });

    let _ = timer_task.await?;
    let _ = walking_task.await?;

    Ok(())
}

#[tokio::main]
async fn only_combat(config: Config) -> io::Result<()> {

    let hp_to_defense_light = config.combat.hp_to_defense_light.clone();
    let hp_to_defense_full = config.combat.hp_to_defense_full.clone();
    let combat_basic = config.skills.basic.clone();
    let combat_start = config.skills.start.clone();
    let combat_combo = config.skills.combo.clone();
    let combat_defense_light = config.skills.defense_light.clone();
    let combat_defense_full = config.skills.defense_full.clone();
    let global_cd = config.combat.global_cd.clone();

    let skills = config.skills.clone();
    let mut cooldown_manager = cooldown_manager_construct(&skills);

    let running = Arc::new(AtomicBool::new(true));
    let r = running.clone();

    ctrlc::set_handler(move || {
        r.store(false, Ordering::SeqCst);
    }).expect("Erro ao definir o manipulador de Ctrl-C");

    let timer_task = task::spawn(run_timer_general(running.clone(), config.timer));
    let hunting_task = task::spawn(async move {
        info!("Starting only combat. Manual walk.");
        while running.load(Ordering::SeqCst) {
            only_combat_instance(unsafe { WINDOW_HANDLE }, &hp_to_defense_light, &hp_to_defense_full, &combat_defense_light, &combat_defense_full, &combat_start, &combat_combo, &combat_basic, global_cd, &mut cooldown_manager);
        }
    });

    let _ = timer_task.await?;
    let _ = hunting_task.await?;

    Ok(())
}

fn cooldown_manager_construct(skills: &Skills) -> CooldownManager {
    let mut cooldown_manager = CooldownManager::new();
    for skill in skills.start.iter(){
        cooldown_manager.set_cooldown(&skill.hotkey, Duration::from_secs(skill.cooldown));
    }
    for skill in skills.combo.iter(){
        cooldown_manager.set_cooldown(&skill.hotkey, Duration::from_secs(skill.cooldown));
    }
    for skill in skills.defense_full.iter(){
        cooldown_manager.set_cooldown(&skill.hotkey, Duration::from_secs(skill.cooldown));
    }
    for skill in skills.defense_light.iter(){
        cooldown_manager.set_cooldown(&skill.hotkey, Duration::from_secs(skill.cooldown));
    }
    cooldown_manager
}

fn main() {
    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();
    if !check_hwid() {
        error!("HWID não corresponde ao proprietário do bot.");
        process::exit(1); // Encerra o programa com um código de status 1
    } else {
        main_menu().expect("Erro ao executar o menu principal");
    }
}