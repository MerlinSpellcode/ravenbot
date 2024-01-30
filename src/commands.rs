use winapi::shared::windef::HWND;
use crate::utils::inputs::{press_w, press_a, press_s, press_d, press_skill, press_tab};
use crate::utils::env::{CDR_Q, VK_F5, VK_F6, VK_Q, CDR_E, VK_E, CDR_1, VK_1, CDR_2, VK_2, CDR_3, VK_3};
use crate::checks::{get_aether, is_hp_below_half, get_target, get_coord};
use std::collections::HashMap;
use std::time::{Instant, Duration};
use std::{thread};

static mut LAST_CALLED: Option<HashMap<String, Instant>> = None;

fn update_last_called(function_id: &str) {
    unsafe {
        let last_called = LAST_CALLED.get_or_insert_with(HashMap::new);
        last_called.insert(function_id.to_string(), Instant::now());
    }
}

fn can_call_function(function_id: &str, cooldown: u64) -> bool {
    unsafe {
        if let Some(last_called) = LAST_CALLED.as_ref() {
            if let Some(instant) = last_called.get(function_id) {
                return instant.elapsed() >= Duration::from_secs(cooldown);
            }
        }
    }
    true
}

fn main_skill_atk(hwnd: HWND) {
    press_skill(hwnd, VK_F5);
    thread::sleep(Duration::from_millis(910));
}

fn sb_burst_skill_atk(hwnd: HWND) {
    if !can_call_function("sb_burst_skill_atk", CDR_Q) {
        return;
    }
    println!("# S1 #");
    update_last_called("sb_burst_skill_atk");
    if check_target(hwnd) {
        loop {
            let aether_value = get_aether();
            check_target(hwnd);
            println!("# S2 #");
            if aether_value > 50.0 {
                println!("# S3 #");
                press_skill(hwnd, VK_F6);
                thread::sleep(Duration::from_secs(2));
                press_skill(hwnd, VK_Q);
                break;
            } else {
                if check_target(hwnd) {
                    println!("# B5 #");
                    main_skill_atk(hwnd);
                    thread::sleep(Duration::from_secs(1));
                } else {
                    println!("# B6 #");
                    break;
                }
            }
        }
    } else {
        return;
    }
    
}

pub fn hp_passive_restore(hwnd: HWND) {
    while is_hp_below_half() {
        combat_instance(hwnd);
        std::thread::sleep(std::time::Duration::from_secs(1));
    }
    println!("HP acima de 50%");
}

pub fn burst_skill_atk(hwnd: HWND, hotkey: u8, cooldown: u64) {
    let skill_cdr_name = format!("{}_skill_atk", hotkey);
    if !can_call_function(&skill_cdr_name, cooldown) {
        return;
    }
    println!("# B1 #");
    update_last_called(&skill_cdr_name);
    if check_target(hwnd) {
        println!("# B2 #");
        loop {
            let aether_value = get_aether();
            check_target(hwnd);
            println!("# B3 #");
            if aether_value > 50.0 {
                println!("# B4 #");
                press_skill(hwnd, hotkey);
                thread::sleep(Duration::from_secs(1));
                break;
            } else {
                if check_target(hwnd) {
                    println!("# B5 #");
                    main_skill_atk(hwnd);
                    thread::sleep(Duration::from_secs(1));
                } else {
                    println!("# B6 #");
                    break;
                }
            }
        }
    } else {
        println!("# B7 #");
        return;
    }
    
}


pub fn walk_to(hwnd: HWND, commands: &[&str]) {
    for &command in commands {
        match command {
            "w" => press_w(hwnd),
            "a" => press_a(hwnd),
            "s" => press_s(hwnd),
            "d" => press_d(hwnd),
            _ => println!("Unknown command: {}", command),
        }
    }
    println!("# WALK TEST #");
}

pub fn path_walker(hwnd: HWND, destination: [i32; 3]) {
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
            combat_instance(hwnd);
            attempts += 1;
        } else {
            // Reseta as tentativas se houver movimento
            attempts = 0;
        }
    }
}


pub fn time_test() {
    thread::sleep(Duration::from_millis(2500));
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


pub fn combat_instance(hwnd: HWND) {
    press_tab(hwnd);
    let mut counter = 0;

    while check_target(hwnd) {
        match counter {
            0 => sb_burst_skill_atk(hwnd),
            1 => burst_skill_atk(hwnd, VK_E, CDR_E),
            2 => burst_skill_atk(hwnd, VK_1, CDR_1),
            3 => burst_skill_atk(hwnd, VK_2, CDR_2),
            4 => {
                burst_skill_atk(hwnd, VK_3, CDR_3);
                counter = -1; // Será incrementado para 0 no final do loop
            }
            _ => (),
        }

        counter += 1;
        thread::sleep(Duration::from_millis(1));
    }
    hp_passive_restore(hwnd);
    println!("# Terminando Combate #");
}