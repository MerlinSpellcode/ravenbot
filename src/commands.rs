use winapi::shared::windef::HWND;
use std::{thread, time::Duration};
use crate::checkstats::{check_aether};
use crate::utils::inputs::{press_w, press_a, press_s, press_d, press_f5, press_e};


pub fn main_skill_atk(hwnd: HWND) {
    press_f5(hwnd);
    thread::sleep(Duration::from_secs(2));
    println!("# Mail Skill #");
}

pub fn burst_skill_atk(hwnd: HWND) {
    main_skill_atk(hwnd);
    press_e(hwnd);
    thread::sleep(Duration::from_secs(1));
    check_aether();
    println!("@@@! Burst Hotkey E !@@@");
}

pub fn move_first_location(hwnd: HWND) {
    press_w(hwnd);
    press_w(hwnd);
    press_w(hwnd);
    press_w(hwnd);
    press_d(hwnd);
    press_s(hwnd);
    press_s(hwnd);
    press_s(hwnd);
    press_s(hwnd);
    press_a(hwnd);
    thread::sleep(Duration::from_secs(10));
    println!("# WALK TEST #");
}

// pub fn main_skill_atk(hwnd: HWND) {
//     unsafe { PostMessageA(hwnd, WM_KEYDOWN, VK_F5 as usize, 0); }
//     unsafe { PostMessageA(hwnd, WM_KEYUP, VK_F5 as usize, 0); }
//     thread::sleep(Duration::from_secs(2));
//     println!("# Mail Skill #");
// }

