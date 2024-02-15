extern crate winapi;
extern crate serde;
extern crate serde_json;

use std::fs;
use std::process;
use std::io::{self, Write};
use winapi::um::winuser::{GetAsyncKeyState, VK_F1};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use winapi::shared::minwindef::{LPARAM, BOOL, DWORD};
use winapi::{
    um::winuser::{EnumWindows, GetWindowThreadProcessId, IsWindowVisible},
    shared::windef::HWND,
};
use std::ptr;
use std::thread;
// use std::time::Duration;

use ravenbot::utils::address::get_base_address;
use ravenbot::utils::env::Config;
use ravenbot::utils::env::Hunt;
use ravenbot::utils::env::Skills;
use ravenbot::utils::env::Combat;
use ravenbot::utils::env::Foods;
use ravenbot::utils::inputs::press_skill;
use ravenbot::commands::combat_instance;
use ravenbot::commands::path_walker;
// use ravenbot::commands::use_foods;
use ravenbot::checks::get_coord;
use ravenbot::checks::check_hwid;

use std::time::Duration;
use tokio::{time::interval, task};

use chrono::{Local, Timelike};

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
            eprintln!("Erro ao encontrar o endereço base do módulo");
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
        eprintln!("Janela não encontrada");
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

async fn run_timer_for_foods(foods: Foods, running: Arc<AtomicBool>) {
    let _hwnd = match get_window_handle() {
        Ok(hwnd) => hwnd,
        Err(err) => {
            eprintln!("Failed to get window handle: {}", err);
            running.store(false, Ordering::SeqCst);
            return;
        }
    };

    let mut interval = interval(Duration::from_secs(foods.timer * 60)); // X minutos baseado no config

    while running.load(Ordering::SeqCst) {
        interval.tick().await;
        use_foods(&foods);
    }
}

fn read_config() -> Config {

    let config_combat: Combat = serde_json::from_str(&fs::read_to_string("config/combat.json")
        .expect("Erro ao ler o arquivo combat.json"))
        .expect("Erro ao deserializar o arquivo combat.json");
    let config_skills: Skills = serde_json::from_str(&fs::read_to_string("config/skills.json")
        .expect("Erro ao ler o arquivo skills.json"))
        .expect("Erro ao deserializar o arquivo skills.json");
    let config_hunts: Vec<Hunt> = serde_json::from_str(&fs::read_to_string("config/hunts.json")
        .expect("Erro ao ler o arquivo hunts.json"))
        .expect("Erro ao deserializar o arquivo hunts.json");
    let config_foods: Foods = serde_json::from_str(&fs::read_to_string("config/foods.json")
        .expect("Erro ao ler o arquivo foods.json"))
        .expect("Erro ao deserializar o arquivo foods.json");
    
    let config_contents = Config {
        hunts: config_hunts,
        combat: config_combat,
        skills: config_skills,
        foods: config_foods
    };


    return config_contents;
}

pub fn use_foods(foods: &Foods){
    let brt = chrono::FixedOffset::west_opt(3 * 3600).unwrap(); // Horário de Brasília (UTC-3)
    let current_time = Local::now().with_timezone(&brt);
    println!("Using foods at {:02}:{:02}:{:02} BRT..", current_time.hour(), current_time.minute(), current_time.second());
    press_skill(unsafe { WINDOW_HANDLE }, &foods.status);
    press_skill(unsafe { WINDOW_HANDLE }, &foods.attack_power);
    press_skill(unsafe { WINDOW_HANDLE }, &foods.hp_mana_regen);
}

fn main_menu() -> io::Result<()> {

    let config = read_config();

    println!("Selecione uma opção:");
    println!("1: Create Hunting Coordinates");
    println!("2: Hunting");
    println!("3: Only Combat (Manual Walk)");

    let mut choice = String::new();
    io::stdin().read_line(&mut choice)?;

    match choice.trim() {
        "1" => create_hunting_coordinates(),
        "2" => hunting(config),
        "3" => only_combat(config),
        _ => {
            println!("Opção inválida, por favor, tente novamente.");
            main_menu()
        },
    }

}

fn create_hunting_coordinates() -> io::Result<()> {
    let running = Arc::new(AtomicBool::new(true));
    let r = running.clone();

    ctrlc::set_handler(move || {
        r.store(false, Ordering::SeqCst);
    }).expect("Erro ao definir o manipulador de Ctrl-C");

    print!("Digite o nome da sua hunt: ");
    io::stdout().flush().unwrap();
    let mut nome = String::new();
    io::stdin().read_line(&mut nome).expect("Falha ao ler a entrada");
    let nome = nome.trim().to_string();

    let file_path = "config/hunts.json";
    let mut config = read_config();

    // Verifica se já existe uma caçada com o nome fornecido, se não, adiciona uma nova
    let hunt_index = config.hunts.iter().position(|h| h.name == nome);
    let _hunt = match hunt_index {
        Some(index) => &mut config.hunts[index],
        None => {
            config.hunts.push(Hunt { name: nome.clone(), route: Vec::new() });
            config.hunts.last_mut().unwrap()
        },
    };

    while running.load(Ordering::SeqCst) {
        unsafe {
            if GetAsyncKeyState(VK_F1 as i32) as u16 & 0x8000 != 0 {
                let current_value = get_coord(); // Supondo que isso retorna um [i32; 3]
                println!("Coordenada adicionada: {:?}", current_value);
    
                // Atualize a rota diretamente sem manter um empréstimo mutável longo
                if let Some(_hunt) = config.hunts.iter_mut().find(|h| h.name == nome) {
                    _hunt.route.push(current_value);
                } else {
                    config.hunts.push(Hunt { name: nome.clone(), route: vec![current_value] });
                }
    
                // Agora que as modificações foram feitas, podemos serializar
                let json_string = serde_json::to_string_pretty(&config.hunts).expect("Falha ao serializar JSON");
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

#[tokio::main]
async fn hunting(config: Config) -> io::Result<()> {

    let hunt_choice = choose_hunt(&config.hunts).expect("Escolha inválida de caçada.");
    let selected_hunt = config.hunts[hunt_choice].clone();
    let hp_regen_passive = config.combat.hp_regen_passive.clone();
    let mana_regen_passive = config.combat.mana_regen_passive.clone();
    let hp_to_defense_light = config.combat.hp_to_defense_light.clone();
    let hp_to_defense_full = config.combat.hp_to_defense_full.clone();
    let combat_basic = config.skills.basic.clone();
    let combat_start = config.skills.start.clone();
    let combat_combo = config.skills.combo.clone();
    let combat_defense_light = config.skills.defense_light.clone();
    let combat_defense_full = config.skills.defense_full.clone();
    let global_cd = config.combat.global_cd.clone();

    let running = Arc::new(AtomicBool::new(true));
    let r = running.clone();

    ctrlc::set_handler(move || {
        r.store(false, Ordering::SeqCst);
    }).expect("Erro ao definir o manipulador de Ctrl-C");

    let food_task = task::spawn(run_timer_for_foods(config.foods.clone(), running.clone()));
    let hunting_task = task::spawn(async move {
        while running.load(Ordering::SeqCst) {
            for path in selected_hunt.route.iter() {
                combat_instance(unsafe { WINDOW_HANDLE }, &hp_regen_passive, &mana_regen_passive, &hp_to_defense_light, &hp_to_defense_full, &combat_defense_light, &combat_defense_full, &combat_start, &combat_combo, &combat_basic, global_cd);
                println!("Going to: {:?}", path);
                path_walker(unsafe { WINDOW_HANDLE }, *path, &hp_regen_passive, &mana_regen_passive, &hp_to_defense_light, &hp_to_defense_full, &combat_basic, &combat_start, &combat_combo, &combat_defense_light, &combat_defense_full, global_cd);
            }
        }
    });

    let _ = food_task.await?;
    let _ = hunting_task.await?;
    
    Ok(())
}

#[tokio::main]
async fn only_combat(config: Config) -> io::Result<()> {

    let hp_regen_passive = config.combat.hp_regen_passive.clone();
    let mana_regen_passive = config.combat.mana_regen_passive.clone();
    let hp_to_defense_light = config.combat.hp_to_defense_light.clone();
    let hp_to_defense_full = config.combat.hp_to_defense_full.clone();
    let combat_basic = config.skills.basic.clone();
    let combat_start = config.skills.start.clone();
    let combat_combo = config.skills.combo.clone();
    let combat_defense_light = config.skills.defense_light.clone();
    let combat_defense_full = config.skills.defense_full.clone();
    let global_cd = config.combat.global_cd.clone();

    let running = Arc::new(AtomicBool::new(true));
    let r = running.clone();

    ctrlc::set_handler(move || {
        r.store(false, Ordering::SeqCst);
    }).expect("Erro ao definir o manipulador de Ctrl-C");

    let hunting_task = task::spawn(async move {
        println!("Starting only combate. Manual walk.");
        while running.load(Ordering::SeqCst) {
            combat_instance(unsafe { WINDOW_HANDLE }, &hp_regen_passive, &mana_regen_passive, &hp_to_defense_light, &hp_to_defense_full, &combat_defense_light, &combat_defense_full, &combat_start, &combat_combo, &combat_basic, global_cd);
        }
    });

    let _ = hunting_task.await?;

    Ok(())
}

fn main() {
    if !check_hwid() {
        println!("HWID não corresponde ao proprietário do bot.");
        process::exit(1); // Encerra o programa com um código de status 1
    } else {
        main_menu().expect("Erro ao executar o menu principal");
    }
}