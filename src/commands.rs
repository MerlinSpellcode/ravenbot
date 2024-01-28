use winapi::shared::windef::HWND;
use crate::checkstats::{check_aether};
use crate::utils::inputs::{press_w, press_a, press_s, press_d, press_skill, press_tab};
use crate::utils::env::{CDR_Q, VK_F5, VK_F6, VK_Q, CDR_E, VK_E, CDR_1, VK_1, CDR_2, VK_2, CDR_3, VK_3};
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

pub fn main_skill_atk(hwnd: HWND) {
    press_skill(hwnd, VK_F5);
    thread::sleep(Duration::from_millis(1550));
    println!("# Mail Skill #");
}

// pub fn sb_burst_skill_atk(hwnd: HWND) {
//     if !can_call_function("sb_burst_skill_atk", CDR_Q) {
//         return;
//     }
//     update_last_called("sb_burst_skill_atk");
//     loop {
//         let aether_value = check_aether();
//         println!("Valor atual de Aether: {}", aether_value);
//         if aether_value > 50 {
//             press_skill(hwnd, VK_F6);
//             thread::sleep(Duration::from_secs(2));
//             press_skill(hwnd, VK_Q);
//             break;
//         } else {
//             main_skill_atk(hwnd);
//             thread::sleep(Duration::from_secs(1));
//         }
//     }
// }

// pub fn burst_skill_atk(hwnd: HWND, hotkey: u8, cooldown: u64) {
//     if !can_call_function("burst_skill_atk", cooldown) {
//         return;
//     }
//     update_last_called("burst_skill_atk");
//     loop {
//         let aether_value = check_aether();
//         println!("Valor atual de Aether: {}", aether_value);
//         if aether_value > 50 {
//             press_skill(hwnd, hotkey);
//             thread::sleep(Duration::from_secs(1));
//             break;
//         } else {
//             main_skill_atk(hwnd);
//             thread::sleep(Duration::from_secs(1));
//         }
//     }
// }

pub fn sb_burst_skill_atk(hwnd: HWND) {
    if !can_call_function("sb_burst_skill_atk", CDR_Q) {
        return;
    }
    main_skill_atk(hwnd);
    main_skill_atk(hwnd);
    press_skill(hwnd, VK_F6);
    thread::sleep(Duration::from_millis(1550));
    press_skill(hwnd, VK_Q);
}

pub fn burst_skill_atk(hwnd: HWND, hotkey: u8, cooldown: u64) {
    update_last_called("burst_skill_atk");
    main_skill_atk(hwnd);
    main_skill_atk(hwnd);
    main_skill_atk(hwnd);
    press_skill(hwnd, hotkey);
    println!("Cooldown: {} seconds", cooldown);
}



pub fn skill_rotation_1(hwnd: HWND) {
    let aether_value = check_aether();
    println!("PRIMEIRO ROTACAO {}", aether_value);
    press_tab(hwnd);
    thread::sleep(Duration::from_millis(2000));
    sb_burst_skill_atk(hwnd);
    press_tab(hwnd);
    burst_skill_atk(hwnd, VK_E, CDR_E);
    burst_skill_atk(hwnd, VK_1, CDR_1);
    main_skill_atk(hwnd);
}

pub fn skill_rotation_2(hwnd: HWND) {
    press_tab(hwnd);
    sb_burst_skill_atk(hwnd);
    press_tab(hwnd);
    burst_skill_atk(hwnd, VK_2, CDR_2);
    burst_skill_atk(hwnd, VK_3, CDR_3);
    burst_skill_atk(hwnd, VK_E, CDR_E);
}

pub fn skill_rotation_3(hwnd: HWND) {
    press_tab(hwnd);
    sb_burst_skill_atk(hwnd);
    press_tab(hwnd);
    burst_skill_atk(hwnd, VK_1, CDR_1);
    main_skill_atk(hwnd);
    thread::sleep(Duration::from_millis(30000));
}




pub fn walk_hunt(hwnd: HWND, commands: &[&str]) {
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

// pub fn main_skill_atk(hwnd: HWND) {
//     unsafe { PostMessageA(hwnd, WM_KEYDOWN, VK_F5 as usize, 0); }
//     unsafe { PostMessageA(hwnd, WM_KEYUP, VK_F5 as usize, 0); }
//     thread::sleep(Duration::from_secs(2));
//     println!("# Mail Skill #");
// }

