use winapi::shared::windef::HWND;
use crate::utils::inputs::{press_w, press_a, press_s, press_d, press_skill, press_tab, double_press_skill};
use crate::utils::env::{BasicS, Skill};
use crate::checks::{get_aether, hp_need_restore, mana_need_restore, is_hp_full, is_mana_full, get_target, get_coord, get_mana_actual, get_hp_actual};

// use tokio::time::{Duration, Instant};
// use std::collections::HashMap;
// use once_cell::sync::Lazy;
// use std::sync::{Arc, Mutex};
// use std::{thread};

// struct CooldownManagerAsync {
//     last_called: Arc<Mutex<HashMap<String, Instant>>>,
// }

// impl CooldownManagerAsync {
//     fn new() -> CooldownManagerAsync {
//         CooldownManagerAsync {
//             last_called: Arc::new(Mutex::new(HashMap::new())),
//         }
//     }

//     fn can_use_skill(&self, function_id: &str, _cooldown: Duration) -> bool {
//         let now = Instant::now();
//         let last_called = self.last_called.lock().unwrap();
    
//         if let Some(&last_time) = last_called.get(function_id) {
//             println!("Habilidade '{}' em cooldown. Agora: {:?}, Último: {:?}, Deve Esperar Até: {:?}", function_id, now, last_time, last_time + _cooldown);
//             now >= last_time
//         } else {
//             true
//         }
//     }

//     // Método assíncrono para atualizar o cooldown
//     async fn update_cooldown(&self, function_id: String, cooldown: Duration) {
//         let now = Instant::now();
//         let next_allowed_time = now + cooldown;
//         println!("Atualizando cooldown para '{}'. Agora: {:?}, Próximo: {:?}", function_id, now, next_allowed_time);
//         let mut last_called = self.last_called.lock().unwrap();
//         last_called.insert(function_id, next_allowed_time);
//     }
// }

// // Definindo a variável estática com Lazy para permitir inicialização em tempo de execução
// static CDR_MANAGER: Lazy<Arc<CooldownManagerAsync>> = Lazy::new(|| {
//     Arc::new(CooldownManagerAsync::new())
// });

pub fn path_walker(hwnd: HWND, destination: [i32; 3], hp_regen_passive: &str, mana_regen_passive: &str, hp_to_defense_light: &str, hp_to_defense_full: &str, combat_basic: &[BasicS], combat_start: &[Skill], combat_combo: &[Skill], combat_defense_light: &[Skill], combat_defense_full: &[Skill], global_cd: u64) {
    let mut attempts = 0;
    let max_attempts = 1000; // Limite para tentativas de movimento para evitar loop infinito

    while attempts < max_attempts {
        let current = get_coord();
        
        // Verifica se o personagem chegou ao destino
        if current == destination {
            break;
        }

        // Tenta mover-se na direção correta
        if current[0] < destination[0] {
            press_d(hwnd); // Aumenta X
        } else if current[0] > destination[0] {
            press_a(hwnd); // Diminui X
        }

        if current[1] < destination[1] {
            press_s(hwnd); // Aumenta Y
        } else if current[1] > destination[1] {
            press_w(hwnd); // Diminui Y
        }

        // Aguarda um curto período antes de tentar novamente
        std::thread::sleep(std::time::Duration::from_millis(1));

        // Verifica se o personagem se moveu
        let new_coord = get_coord();
        if new_coord == current {
            // O personagem não se moveu, então tenta outro movimento
            combat_instance(hwnd, hp_regen_passive, mana_regen_passive, hp_to_defense_light, hp_to_defense_full, combat_defense_light, combat_defense_full, combat_start, combat_combo, combat_basic, global_cd);
            attempts += 1;
        } else {
            // Reseta as tentativas se houver movimento
            attempts = 0;
        }
    }
}


fn check_target(hwnd: HWND) -> bool {
    let first_try_get = get_target();
    if first_try_get {
        true
    } else {
        press_tab(hwnd);
        let sec_try_get = get_target();
        if sec_try_get {
            true
        } else {
            press_tab(hwnd);
            let third_try_get = get_target();
            if third_try_get {
                true
            } else {
                false
            }
        }
    }
}

fn sleep_for_global_cd(skill: &Skill, global_cd: u64){
    if skill.has_global {
        println!("Waiting for Global Cooldown ({}ms).", global_cd);
        std::thread::sleep(std::time::Duration::from_millis(global_cd));
    }
}

fn start_fight(hwnd: HWND, combat_start: &[Skill], combat_basic: &[BasicS], global_cd: u64) {
    println!("Begining 'Start' skills rotation.");
    for skill in combat_start.iter(){
        if check_target(hwnd){
            if skill.prereq != "" {
                println!("Casting hotkey ({}), prereq of {} ({}).", skill.prereq, skill.name, skill.hotkey);
                press_skill(hwnd, &skill.prereq);
                sleep_for_global_cd(skill, global_cd);
            }
            if skill.aether {
                if get_aether() > 49.0 {
                    if skill.is_area {
                        println!("Casting area skill {} ({}) with {} Aether.", skill.name, skill.hotkey, get_aether());
                        double_press_skill(hwnd, &skill.hotkey);
                    } else {
                        println!("Casting skill {} ({}) with {} Aether.", skill.name, skill.hotkey, get_aether());
                        press_skill(hwnd, &skill.hotkey);
                    }
                    sleep_for_global_cd(skill, global_cd);
                } else {
                    generate_aether(hwnd, combat_basic, global_cd);
                    if skill.is_area {
                        println!("Casting area skill {} ({}) with {} Aether.", skill.name, skill.hotkey, get_aether());
                        double_press_skill(hwnd, &skill.hotkey);
                    } else {
                        println!("Casting skill {} ({}) with {} Aether.", skill.name, skill.hotkey, get_aether());
                        press_skill(hwnd, &skill.hotkey);
                    }
                    sleep_for_global_cd(skill, global_cd);
                }
            } else {
                if skill.is_area {
                    println!("Casting area skill {} ({}).", skill.name, skill.hotkey);
                    double_press_skill(hwnd, &skill.hotkey);
                } else {
                    println!("Casting skill {} ({}).", skill.name, skill.hotkey);
                    press_skill(hwnd, &skill.hotkey);
                }
                sleep_for_global_cd(skill, global_cd);
            }
        } else {
            println!("No more targets on sight.");
            break;
        }
    }
}

fn generate_aether(hwnd: HWND, combat_basic: &[BasicS], global_cd: u64){
    while get_aether() < 50.0 && check_target(hwnd) {
        println!("Generating Aether with {} ({}).", combat_basic[0].name, combat_basic[0].hotkey);
        press_skill(hwnd, &combat_basic[0].hotkey);
        println!("Waiting for Global Cooldown ({}ms).", global_cd);
        std::thread::sleep(std::time::Duration::from_millis(global_cd));
        if get_aether() > 49.0 {
            break;
        }
    }
}

fn defensive_skills(hwnd: HWND, hp_to_defense_light: &str, hp_to_defense_full: &str, combat_defense_light: &[Skill], combat_defense_full: &[Skill], combat_basic: &[BasicS], global_cd: u64){
    if hp_need_restore(hp_to_defense_full){
        get_hp_actual();
        println!("HP on critical situation. Starting full defensive skills.");
        for skill in combat_defense_full.iter(){
            if check_target(hwnd){
                if skill.prereq != "" {
                    println!("Casting hotkey ({}), prereq of {} ({}).", skill.prereq, skill.name, skill.hotkey);
                    press_skill(hwnd, &skill.prereq);
                    sleep_for_global_cd(skill, global_cd);
                }
                if skill.aether {
                    if get_aether() > 49.0 {
                        if skill.is_area {
                            println!("Casting area skill {} ({}) with {} Aether.", skill.name, skill.hotkey, get_aether());
                            double_press_skill(hwnd, &skill.hotkey);
                        } else {
                            println!("Casting skill {} ({}) with {} Aether.", skill.name, skill.hotkey, get_aether());
                            press_skill(hwnd, &skill.hotkey);
                        }
                        sleep_for_global_cd(skill, global_cd);
                    } else {
                        generate_aether(hwnd, combat_basic, global_cd);
                        if skill.is_area {
                            println!("Casting area skill {} ({}) with {} Aether.", skill.name, skill.hotkey, get_aether());
                            double_press_skill(hwnd, &skill.hotkey);
                        } else {
                            println!("Casting skill {} ({}) with {} Aether.", skill.name, skill.hotkey, get_aether());
                            press_skill(hwnd, &skill.hotkey);
                        }
                        sleep_for_global_cd(skill, global_cd);
                    }
                } else {
                    if skill.is_area {
                        println!("Casting area skill {} ({}).", skill.name, skill.hotkey);
                        double_press_skill(hwnd, &skill.hotkey);
                    } else {
                        println!("Casting skill {} ({}).", skill.name, skill.hotkey);
                        press_skill(hwnd, &skill.hotkey);
                    }
                    sleep_for_global_cd(skill, global_cd);
                }
            } else {
                println!("No more targets on sight.");
                break;
            }
        }
    } else if hp_need_restore(hp_to_defense_light) {
        get_hp_actual();
        println!("HP on hard situation. Starting light defensive skills.");
        for skill in combat_defense_light.iter(){
            if check_target(hwnd){
                if skill.prereq != "" {
                    println!("Casting hotkey ({}), prereq of {} ({}).", skill.prereq, skill.name, skill.hotkey);
                    press_skill(hwnd, &skill.prereq);
                    sleep_for_global_cd(skill, global_cd);
                }
                if skill.aether {
                    if get_aether() > 49.0 {
                        if skill.is_area {
                            println!("Casting area skill {} ({}) with {} Aether.", skill.name, skill.hotkey, get_aether());
                            double_press_skill(hwnd, &skill.hotkey);
                        } else {
                            println!("Casting skill {} ({}) with {} Aether.", skill.name, skill.hotkey, get_aether());
                            press_skill(hwnd, &skill.hotkey);
                        }
                        sleep_for_global_cd(skill, global_cd);
                    } else {
                        generate_aether(hwnd, combat_basic, global_cd);
                        if skill.is_area {
                            println!("Casting area skill {} ({}) with {} Aether.", skill.name, skill.hotkey, get_aether());
                            double_press_skill(hwnd, &skill.hotkey);
                        } else {
                            println!("Casting skill {} ({}) with {} Aether.", skill.name, skill.hotkey, get_aether());
                            press_skill(hwnd, &skill.hotkey);
                        }
                        sleep_for_global_cd(skill, global_cd);
                    }
                } else {
                    if skill.is_area {
                        println!("Casting area skill {} ({}).", skill.name, skill.hotkey);
                        double_press_skill(hwnd, &skill.hotkey);
                    } else {
                        println!("Casting skill {} ({}).", skill.name, skill.hotkey);
                        press_skill(hwnd, &skill.hotkey);
                    }
                    sleep_for_global_cd(skill, global_cd);
                }
            } else {
                println!("No more targets on sight.");
                break;
            }
        }
    }
}

fn combo_skills(hwnd: HWND, hp_to_defense_light: &str, hp_to_defense_full: &str, combat_defense_light:&[Skill], combat_defense_full:&[Skill], combat_combo: &[Skill], combat_basic: &[BasicS], global_cd: u64){
    println!("Begining 'Combo' skills rotation.");
    for skill in combat_combo.iter(){
        if check_target(hwnd){
            defensive_skills(hwnd, hp_to_defense_light, hp_to_defense_full, combat_defense_light, combat_defense_full, combat_basic, global_cd);
            if skill.prereq != "" {
                println!("Casting hotkey ({}), prereq of {} ({}).", skill.prereq, skill.name, skill.hotkey);
                press_skill(hwnd, &skill.prereq);
                sleep_for_global_cd(skill, global_cd);
            }
            if skill.aether {
                if get_aether() > 49.0 {
                    if check_target(hwnd) {
                        if skill.is_area {
                            println!("Casting area skill {} ({}) with {} Aether.", skill.name, skill.hotkey, get_aether());
                            double_press_skill(hwnd, &skill.hotkey);
                        } else {
                            println!("Casting skill {} ({}) with {} Aether.", skill.name, skill.hotkey, get_aether());
                            press_skill(hwnd, &skill.hotkey);
                        }
                        sleep_for_global_cd(skill, global_cd);
                    } else {
                        break;
                    }
                } else {
                    generate_aether(hwnd, combat_basic, global_cd);
                    if skill.is_area {
                        println!("Casting area skill {} ({}) with {} Aether.", skill.name, skill.hotkey, get_aether());
                        double_press_skill(hwnd, &skill.hotkey);
                    } else {
                        println!("Casting skill {} ({}) with {} Aether.", skill.name, skill.hotkey, get_aether());
                        press_skill(hwnd, &skill.hotkey);
                    }
                    sleep_for_global_cd(skill, global_cd);
                }
            } else {
                std::thread::sleep(std::time::Duration::from_millis(global_cd));
                if skill.is_area {
                    println!("Casting area skill {} ({}).", skill.name, skill.hotkey);
                    double_press_skill(hwnd, &skill.hotkey);
                } else {
                    println!("Casting skill {} ({}).", skill.name, skill.hotkey);
                    press_skill(hwnd, &skill.hotkey);
                }
            }
        } else {
            println!("No more targets on sight.");
            break;
        }
    }
}

pub fn combat_instance(hwnd: HWND, hp_regen_passive: &str, mana_regen_passive: &str, hp_to_defense_light: &str, hp_to_defense_full: &str, combat_defense_light:&[Skill], combat_defense_full:&[Skill], combat_start: &[Skill], combat_combo: &[Skill], combat_basic: &[BasicS], global_cd: u64) {
    while check_target(hwnd) {
        println!("Target found. Starting FIGHT.");
        std::thread::sleep(std::time::Duration::from_millis(2100));
        defensive_skills(hwnd, hp_to_defense_light, hp_to_defense_full, combat_defense_light, combat_defense_full, combat_basic, global_cd);
        start_fight(hwnd, combat_start, combat_basic, global_cd);
        generate_aether(hwnd, combat_basic, global_cd);
        combo_skills(hwnd, hp_to_defense_light, hp_to_defense_full, combat_defense_light, combat_defense_full, combat_combo, combat_basic, global_cd);
    }
    
    if hp_need_restore(hp_regen_passive) {
        get_hp_actual();
        println!("HP needs passive restore.");
        while is_hp_full() {
            std::thread::sleep(std::time::Duration::from_secs(1));
            if check_target(hwnd) {
                break;
            }
        }
    }
    if mana_need_restore(mana_regen_passive) {
        get_mana_actual();
        println!("Mana needs passive restore.");
        while is_mana_full() {
            std::thread::sleep(std::time::Duration::from_secs(1));
            if check_target(hwnd) {
                break;
            }
        }
    }
    return;
}