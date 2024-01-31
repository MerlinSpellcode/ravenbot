use winapi::shared::windef::HWND;
use crate::utils::inputs::{press_w, press_a, press_s, press_d, press_skill, press_tab};
use crate::utils::env::{VK_F5, VK_F6, VK_Q, VK_E, VK_1, VK_2, VK_3, VK_F9, VK_R, VK_Z, VK_C, VK_4, CDR_COMBO_1, CDR_COMBO_2};
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
    thread::sleep(Duration::from_millis(1550));
}

fn combo_aether_full(hwnd: HWND, hotkey1: u8, hotkey2: u8, cooldown: u64) {
    if !can_call_function("combo_aether_full", cooldown) {
        return;
    }
    println!("# S1 #");
    update_last_called("combo_aether_full");
    if check_target(hwnd) {
        loop {
            let aether_value = get_aether();
            check_target(hwnd);
            if aether_value > 99.0 {
                press_skill(hwnd, hotkey1);
                std::thread::sleep(std::time::Duration::from_millis(1550));
                press_skill(hwnd, hotkey2);
                std::thread::sleep(std::time::Duration::from_millis(1550));
                break;
            } else {
                if check_target(hwnd) {
                    main_skill_atk(hwnd);
                    std::thread::sleep(std::time::Duration::from_millis(1550));
                } else {
                    break;
                }
            }
        }
    } else {
        return;
    }
    
}

fn combo(hwnd: HWND, hotkey1: u8, hotkey2: u8, cooldown: u64) {
    if !can_call_function("combo", cooldown) {
        return;
    }
    println!("# S1 #");
    update_last_called("combo");
    if check_target(hwnd) {
        loop {
            let aether_value = get_aether();
            check_target(hwnd);
            if aether_value > 50.0 {
                press_skill(hwnd, VK_R);
                std::thread::sleep(std::time::Duration::from_millis(550));
                press_skill(hwnd, hotkey1);
                std::thread::sleep(std::time::Duration::from_millis(1550));
                press_skill(hwnd, hotkey2);
                std::thread::sleep(std::time::Duration::from_millis(1550));
                break;
            } else {
                if check_target(hwnd) {
                    main_skill_atk(hwnd);
                    std::thread::sleep(std::time::Duration::from_millis(1550));
                } else {
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
        if check_target(hwnd) {
            break;
        }
    }
    println!("HP acima de 50%");
    return;
}

pub fn skill_atk(hwnd: HWND, hotkey: u8) {
    if check_target(hwnd) {
        loop {
            let aether_value = get_aether();
            check_target(hwnd);
            if aether_value > 50.0 {
                press_skill(hwnd, hotkey);
                std::thread::sleep(std::time::Duration::from_millis(1550));
                break;
            } else {
                if check_target(hwnd) {
                    main_skill_atk(hwnd);
                    std::thread::sleep(std::time::Duration::from_millis(1550));
                } else {
                    break;
                }
            }
        }
    } else {
        return;
    }
    
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
    let mut counter = 0;

    while check_target(hwnd) {
        match counter {
            0 => skill_atk(hwnd, VK_1),
            1 => combo(hwnd, VK_C, VK_F6, CDR_COMBO_2),
            2 => combo_aether_full(hwnd, VK_F9, VK_Q, CDR_COMBO_1),
            3 => skill_atk(hwnd, VK_2),
            4 => skill_atk(hwnd, VK_3),
            5 => skill_atk(hwnd, VK_4),
            6 => skill_atk(hwnd, VK_E),
            7 => {
                skill_atk(hwnd, VK_Z);
                counter = -1; // Será incrementado para 0 no final do loop
            }
            _ => (),
        }

        counter += 1;
        thread::sleep(Duration::from_millis(1));
    }
    if is_hp_below_half() {
        while is_hp_below_half() {
            std::thread::sleep(std::time::Duration::from_secs(10));
        }
    }
    println!("# Saindo do Combat Stance #");
    return;
}