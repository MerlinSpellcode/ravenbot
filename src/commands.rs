use log2::*;
use winapi::shared::windef::HWND;
use crate::utils::inputs::{press_w, press_a, press_s, press_d, press_skill, press_tab, double_press_skill};
use crate::utils::env::{BasicS, Drink, Hunt, Prereq, Skill, CooldownManager};
use crate::checks::{get_aether, get_coord, get_hp_actual, get_mana_actual, get_target, hp_can_continue, hp_need_combat_restore, hp_need_drink, hp_need_passive_restore, mana_can_continue, mana_need_drink, mana_need_passive_restore};

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
    if skill.has_global{
        info!("Waiting for Global Cooldown ({}ms).", global_cd);
        std::thread::sleep(std::time::Duration::from_millis(global_cd));
    }
}

fn prereq_sleep_for_global_cd(prereq: &Prereq, global_cd: u64){
    if prereq.has_global{
        info!("Waiting for Global Cooldown ({}ms).", global_cd);
        std::thread::sleep(std::time::Duration::from_millis(global_cd));
    }
}

fn generate_aether(hwnd: HWND, combat_basic: &[BasicS], global_cd: u64){
    if check_target(hwnd) {
        while get_aether() < 50.0 {
            info!("Generating Aether with {} ({}).", combat_basic[0].name, combat_basic[0].hotkey);
            press_skill(hwnd, &combat_basic[0].hotkey);
            info!("Waiting for Global Cooldown ({}ms).", global_cd);
            std::thread::sleep(std::time::Duration::from_millis(global_cd));
            if get_aether() > 49.0 || !check_target(hwnd) {
                break;
            }
        }
    } 
}

fn cast_skill_prereq(hwnd: HWND, skill: &Skill, global_cd: u64, combat_basic: &[BasicS], cooldown_manager: &mut CooldownManager){
    if skill.prereq.hotkey != "" {
        info!("{} ({}) is a prereq of {} ({}).", skill.prereq.name, skill.prereq.hotkey, skill.name, skill.hotkey);
        if skill.prereq.aether {
            if get_aether() > 49.0 {
                if skill.prereq.is_area {
                    info!("Casting area skill {} ({}) with {} Aether.", skill.prereq.name, skill.prereq.hotkey, get_aether());
                    if cooldown_manager.execute_action(&skill.prereq.hotkey){
                        info!("Area skill {} ({}) is not on cooldown. Can be casted.", skill.prereq.name, skill.prereq.hotkey);
                        double_press_skill(hwnd, &skill.prereq.hotkey);
                        prereq_sleep_for_global_cd(&skill.prereq, global_cd);
                    } else {
                        info!("Area skill {} ({}) is on cooldown. Cannot be casted.", skill.prereq.name, skill.prereq.hotkey);
                    }
                } else {
                    info!("Casting skill {} ({}) with {} Aether.", skill.prereq.name, skill.prereq.hotkey, get_aether());
                    if cooldown_manager.execute_action(&skill.prereq.hotkey){
                        info!("Skill {} ({}) is not on cooldown. Can be casted.", skill.prereq.name, skill.prereq.hotkey);
                        press_skill(hwnd, &skill.prereq.hotkey);
                        prereq_sleep_for_global_cd(&skill.prereq, global_cd);
                    } else {
                        info!("Skill {} ({}) is on cooldown. Cannot be casted.", skill.prereq.name, skill.prereq.hotkey);
                    }
                }
            } else {
                generate_aether(hwnd, combat_basic, global_cd);
                if skill.prereq.is_area {
                    info!("Casting area skill {} ({}) with {} Aether.", skill.prereq.name, skill.prereq.hotkey, get_aether());
                    if cooldown_manager.execute_action(&skill.prereq.hotkey){
                        info!("Area skill {} ({}) is not on cooldown. Can be casted.", skill.prereq.name, skill.prereq.hotkey);
                        double_press_skill(hwnd, &skill.prereq.hotkey);
                        prereq_sleep_for_global_cd(&skill.prereq, global_cd);
                    } else {
                        info!("Area skill {} ({}) is on cooldown. Cannot be casted.", skill.prereq.name, skill.prereq.hotkey);
                    }
                } else {
                    info!("Casting skill {} ({}) with {} Aether.", skill.prereq.name, skill.prereq.hotkey, get_aether());
                    if cooldown_manager.execute_action(&skill.prereq.hotkey){
                        info!("Skill {} ({}) is not on cooldown. Can be casted.", skill.prereq.name, skill.prereq.hotkey);
                        press_skill(hwnd, &skill.prereq.hotkey);
                        prereq_sleep_for_global_cd(&skill.prereq, global_cd);
                    } else {
                        info!("Skill {} ({}) is on cooldown. Cannot be casted.", skill.prereq.name, skill.prereq.hotkey);
                    }
                }
            }
        } else {
            if skill.prereq.is_area {
                info!("Casting area skill {} ({}) with {} Aether.", skill.prereq.name, skill.prereq.hotkey, get_aether());
                if cooldown_manager.execute_action(&skill.prereq.hotkey){
                    info!("Area skill {} ({}) is not on cooldown. Can be casted.", skill.prereq.name, skill.prereq.hotkey);
                    double_press_skill(hwnd, &skill.prereq.hotkey);
                    prereq_sleep_for_global_cd(&skill.prereq, global_cd);
                } else {
                    info!("Area skill {} ({}) is on cooldown. Cannot be casted.", skill.prereq.name, skill.prereq.hotkey);
                }
            } else {
                info!("Casting skill {} ({}) with {} Aether.", skill.prereq.name, skill.prereq.hotkey, get_aether());
                if cooldown_manager.execute_action(&skill.prereq.hotkey){
                    info!("Skill {} ({}) is not on cooldown. Can be casted.", skill.prereq.name, skill.prereq.hotkey);
                    press_skill(hwnd, &skill.prereq.hotkey);
                    prereq_sleep_for_global_cd(&skill.prereq, global_cd);
                } else {
                    info!("Skill {} ({}) is on cooldown. Cannot be casted.", skill.prereq.name, skill.prereq.hotkey);
                }
            }
        }
    }
}

fn cast_skill(hwnd: HWND, skill: &Skill, global_cd: u64, combat_basic: &[BasicS], cooldown_manager: &mut CooldownManager){
    if skill.aether {
        if get_aether() > 49.0 {
            if skill.is_area {
                info!("Casting area skill {} ({}) with {} Aether.", skill.name, skill.hotkey, get_aether());
                if cooldown_manager.execute_action(&skill.hotkey){
                    info!("Area skill {} ({}) is not on cooldown. Can be casted.", skill.name, skill.hotkey);
                    double_press_skill(hwnd, &skill.hotkey);
                    sleep_for_global_cd(&skill, global_cd);
                } else {
                    info!("Area skill {} ({}) is on cooldown. Cannot be casted.", skill.name, skill.hotkey);
                }
            } else {
                info!("Casting skill {} ({}) with {} Aether.", skill.name, skill.hotkey, get_aether());
                if cooldown_manager.execute_action(&skill.hotkey){
                    info!("Skill {} ({}) is not on cooldown. Can be casted.", skill.name, skill.hotkey);
                    press_skill(hwnd, &skill.hotkey);
                    sleep_for_global_cd(&skill, global_cd);
                } else {
                    info!("Skill {} ({}) is on cooldown. Cannot be casted.", skill.name, skill.hotkey);
                }
            }
        } else {
            generate_aether(hwnd, combat_basic, global_cd);
            if skill.is_area {
                info!("Casting area skill {} ({}) with {} Aether.", skill.name, skill.hotkey, get_aether());
                if cooldown_manager.execute_action(&skill.hotkey){
                    info!("Area skill {} ({}) is not on cooldown. Can be casted.", skill.name, skill.hotkey);
                    double_press_skill(hwnd, &skill.hotkey);
                    sleep_for_global_cd(&skill, global_cd);
                } else {
                    info!("Area skill {} ({}) is on cooldown. Cannot be casted.", skill.name, skill.hotkey);
                }
            } else {
                info!("Casting skill {} ({}) with {} Aether.", skill.name, skill.hotkey, get_aether());
                if cooldown_manager.execute_action(&skill.hotkey){
                    info!("Skill {} ({}) is not on cooldown. Can be casted.", skill.name, skill.hotkey);
                    press_skill(hwnd, &skill.hotkey);
                    sleep_for_global_cd(&skill, global_cd);
                } else {
                    info!("Skill {} ({}) is on cooldown. Cannot be casted.", skill.name, skill.hotkey);
                }
            }
        }
    } else {
        if skill.is_area {
            info!("Casting area skill {} ({}).", skill.name, skill.hotkey);
            if cooldown_manager.execute_action(&skill.hotkey){
                info!("Area skill {} ({}) is not on cooldown. Can be casted.", skill.name, skill.hotkey);
                double_press_skill(hwnd, &skill.hotkey);
                sleep_for_global_cd(&skill, global_cd);
            } else {
                info!("Area skill {} ({}) is on cooldown. Cannot be casted.", skill.name, skill.hotkey);
            }
        } else {
            info!("Casting skill {} ({}).", skill.name, skill.hotkey);
            if cooldown_manager.execute_action(&skill.hotkey){
                info!("Skill {} ({}) is not on cooldown. Can be casted.", skill.name, skill.hotkey);
                press_skill(hwnd, &skill.hotkey);
                sleep_for_global_cd(&skill, global_cd);
            } else {
                info!("Skill {} ({}) is on cooldown. Cannot be casted.", skill.name, skill.hotkey);
            }
        }
    }
}

fn defensive_skills(hwnd: HWND, hp_to_defense_light: &str, hp_to_defense_full: &str, combat_defense_light:&[Skill], combat_defense_full:&[Skill], combat_basic: &[BasicS], global_cd: u64, cooldown_manager: &mut CooldownManager){
    if hp_need_combat_restore(hp_to_defense_full){
        get_hp_actual();
        info!("HP on critical situation. Starting full defensive skills.");
        for skill in combat_defense_full.iter(){
            if check_target(hwnd){
                cast_skill_prereq(hwnd, skill, global_cd, combat_basic, cooldown_manager);
                cast_skill(hwnd, skill, global_cd, combat_basic, cooldown_manager);
            } else {
                info!("No more targets on sight.");
                break;
            }
        }
    } else if hp_need_combat_restore(hp_to_defense_light) {
        get_hp_actual();
        info!("HP on hard situation. Starting light defensive skills.");
        for skill in combat_defense_light.iter(){
            if check_target(hwnd){
                cast_skill_prereq(hwnd, skill, global_cd, combat_basic, cooldown_manager);
                cast_skill(hwnd, skill, global_cd, combat_basic, cooldown_manager);
            } else {
                info!("No more targets on sight.");
                break;
            }
        }
    }
}

fn start_fight(hwnd: HWND, hp_to_defense_light: &str, hp_to_defense_full: &str, combat_defense_light:&[Skill], combat_defense_full:&[Skill], combat_basic: &[BasicS], global_cd: u64, combat_start: &[Skill], cooldown_manager: &mut CooldownManager){
    info!("Begining 'Start' skills rotation.");
    for skill in combat_start.iter(){
        if check_target(hwnd){
            defensive_skills(hwnd, hp_to_defense_light, hp_to_defense_full, combat_defense_light, combat_defense_full, combat_basic, global_cd, cooldown_manager);
            cast_skill_prereq(hwnd, skill, global_cd, combat_basic, cooldown_manager);
            cast_skill(hwnd, skill, global_cd, combat_basic, cooldown_manager);
        } else {
            info!("No more targets on sight.");
            break;
        }
    }
}

fn combo_skills(hwnd: HWND, hp_to_defense_light: &str, hp_to_defense_full: &str, combat_defense_light:&[Skill], combat_defense_full:&[Skill], combat_combo: &[Skill], combat_basic: &[BasicS], global_cd: u64, cooldown_manager: &mut CooldownManager){
    info!("Begining 'Combo' skills rotation.");
    for skill in combat_combo.iter(){
        if check_target(hwnd){
            defensive_skills(hwnd, hp_to_defense_light, hp_to_defense_full, combat_defense_light, combat_defense_full, combat_basic, global_cd, cooldown_manager);
            cast_skill_prereq(hwnd, skill, global_cd, combat_basic, cooldown_manager);
            cast_skill(hwnd, skill, global_cd, combat_basic, cooldown_manager);
        } else {
            info!("No more targets on sight.");
            break;
        }
    }
}

pub fn hunting_path_walker(hwnd: HWND, destination: [i32; 3], hp_regen_passive: &str, mana_regen_passive: &str, hp_to_defense_light: &str, hp_to_defense_full: &str, combat_basic: &[BasicS], combat_start: &[Skill], combat_combo: &[Skill], combat_defense_light: &[Skill], combat_defense_full: &[Skill], global_cd: u64, drink: &Drink, selected_hunt: &Hunt, hp_to_continue: &str, mana_to_continue: &str, cooldown_manager: &mut CooldownManager) {
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
            hunting_instance(hwnd, hp_regen_passive, mana_regen_passive, hp_to_defense_light, hp_to_defense_full, combat_defense_light, combat_defense_full, combat_start, combat_combo, combat_basic, global_cd, drink, selected_hunt, hp_to_continue, mana_to_continue, cooldown_manager);
            attempts += 1;
        } else {
            // Reseta as tentativas se houver movimento
            attempts = 0;
        }
    }
}

pub fn only_walk_path_walker(hwnd: HWND, destination: [i32; 3]) {
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
            attempts += 1;
        } else {
            // Reseta as tentativas se houver movimento
            attempts = 0;
        }
    }
}

pub fn hunting_instance(hwnd: HWND, hp_regen_passive: &str, mana_regen_passive: &str, hp_to_defense_light: &str, hp_to_defense_full: &str, combat_defense_light:&[Skill], combat_defense_full:&[Skill], combat_start: &[Skill], combat_combo: &[Skill], combat_basic: &[BasicS], global_cd: u64, drink: &Drink, selected_hunt: &Hunt, hp_to_continue: &str, mana_to_continue: &str, cooldown_manager: &mut CooldownManager) {
    while check_target(hwnd) {
        info!("Target found. Starting FIGHT.");
        if selected_hunt.stairs {
            std::thread::sleep(std::time::Duration::from_millis(2100));
        }
        defensive_skills(hwnd, hp_to_defense_light, hp_to_defense_full, combat_defense_light, combat_defense_full, combat_basic, global_cd, cooldown_manager);
        start_fight(hwnd, hp_to_defense_light, hp_to_defense_full, combat_defense_light, combat_defense_full, combat_basic, global_cd, combat_start, cooldown_manager);
        // defensive_skills(hwnd, hp_to_defense_light, hp_to_defense_full, combat_defense_light, combat_defense_full, combat_basic, global_cd, cooldown_manager);
        combo_skills(hwnd, hp_to_defense_light, hp_to_defense_full, combat_defense_light, combat_defense_full, combat_combo, combat_basic, global_cd, cooldown_manager);
    }
    
    if hp_need_passive_restore(hp_regen_passive) {
        get_hp_actual();
        info!("HP needs passive restore.");
        while hp_can_continue(hp_to_continue) {
            if check_target(hwnd) {
                break;
            } else {
                std::thread::sleep(std::time::Duration::from_secs(1));
                if hp_need_drink(&drink.hp_to_use) {
                    if check_target(hwnd) {
                        break;
                    }
                    info!("Using drink ({}) to regenerate HP.", drink.hotkey);
                    press_skill(hwnd, &drink.hotkey)
                }
            }
        }
    }
    if mana_need_passive_restore(mana_regen_passive) {
        get_mana_actual();
        info!("Mana needs passive restore.");
        while mana_can_continue(mana_to_continue) {
            if check_target(hwnd) {
                break;
            } else {
                std::thread::sleep(std::time::Duration::from_secs(1));
                if mana_need_drink(&drink.mana_to_use) {
                    if check_target(hwnd) {
                        break;
                    }
                    info!("Using drink ({}) to regenerate Mana.", drink.hotkey);
                    press_skill(hwnd, &drink.hotkey)
                }
            }
        }
    }
    return;
}

pub fn only_combat_instance(hwnd: HWND, hp_to_defense_light: &str, hp_to_defense_full: &str, combat_defense_light:&[Skill], combat_defense_full:&[Skill], combat_start: &[Skill], combat_combo: &[Skill], combat_basic: &[BasicS], global_cd: u64, cooldown_manager: &mut CooldownManager) {
    while check_target(hwnd) {
        info!("Target found. Starting FIGHT.");
        defensive_skills(hwnd, hp_to_defense_light, hp_to_defense_full, combat_defense_light, combat_defense_full, combat_basic, global_cd, cooldown_manager);
        start_fight(hwnd, hp_to_defense_light, hp_to_defense_full, combat_defense_light, combat_defense_full, combat_basic, global_cd, combat_start, cooldown_manager);
        // defensive_skills(hwnd, hp_to_defense_light, hp_to_defense_full, combat_defense_light, combat_defense_full, combat_basic, global_cd, cooldown_manager);
        combo_skills(hwnd, hp_to_defense_light, hp_to_defense_full, combat_defense_light, combat_defense_full, combat_combo, combat_basic, global_cd, cooldown_manager);
    }
    return;
}