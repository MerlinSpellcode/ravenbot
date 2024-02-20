use log::warn;
use winapi::um::winuser::{PostMessageA, WM_KEYDOWN, WM_KEYUP};
use winapi::shared::windef::HWND;
use std::thread;
use std::time::Duration;
use crate::utils::env::{VK_W, VK_A, VK_S, VK_D, VK_TAB, HOTKEYS};

pub fn press_w(hwnd: HWND) {
    unsafe { PostMessageA(hwnd, WM_KEYDOWN, VK_W as usize, 0); }
    unsafe { PostMessageA(hwnd, WM_KEYUP, VK_W as usize, 0); }
    thread::sleep(Duration::from_millis(1));
}
pub fn press_a(hwnd: HWND) {
    unsafe { PostMessageA(hwnd, WM_KEYDOWN, VK_A as usize, 0); }
    unsafe { PostMessageA(hwnd, WM_KEYUP, VK_A as usize, 0); }
    thread::sleep(Duration::from_millis(1));
}
pub fn press_s(hwnd: HWND) {
    unsafe { PostMessageA(hwnd, WM_KEYDOWN, VK_S as usize, 0); }
    unsafe { PostMessageA(hwnd, WM_KEYUP, VK_S as usize, 0); }
    thread::sleep(Duration::from_millis(1));
}
pub fn press_d(hwnd: HWND) {
    unsafe { PostMessageA(hwnd, WM_KEYDOWN, VK_D as usize, 0); }
    unsafe { PostMessageA(hwnd, WM_KEYUP, VK_D as usize, 0); }
    thread::sleep(Duration::from_millis(1));
}
pub fn press_tab(hwnd: HWND) {
    unsafe { PostMessageA(hwnd, WM_KEYDOWN, VK_TAB as usize, 0); }
    unsafe { PostMessageA(hwnd, WM_KEYUP, VK_TAB as usize, 0); }
    thread::sleep(Duration::from_millis(10));
}
// @@@@@@@@@@@@@@@@@@@@@@@@@@@@@@
// @@@@@@@@@@@@@@@@@@@@@@@@@@@@@@
// @@@@@@@@@@@@@@@@@@@@@@@@@@@@@@
// @@@@@@@@@@@@@@@@@@@@@@@@@@@@@@
// @@@@@@@@@@@@@@@@@@@@@@@@@@@@@@
// @@@@@@@@@@@@@@@@@@@@@@@@@@@@@@
pub fn press_skill(hwnd: HWND, key: &str) {
    match HOTKEYS.get(key) {
        Some(hotkey) => {
            unsafe { PostMessageA(hwnd, WM_KEYDOWN, *hotkey as usize, 0); }
            thread::sleep(Duration::from_millis(100));
            unsafe { PostMessageA(hwnd, WM_KEYUP, *hotkey as usize, 0); }
            thread::sleep(Duration::from_millis(100));
        }
        None => {
            warn!("Hotkey not found for: {}", key);
        }
    }
}

pub fn double_press_skill(hwnd: HWND, key: &str){
    match HOTKEYS.get(key) {
        Some(hotkey) => {
            unsafe { PostMessageA(hwnd, WM_KEYDOWN, *hotkey as usize, 0); }
            thread::sleep(Duration::from_millis(100));
            unsafe { PostMessageA(hwnd, WM_KEYUP, *hotkey as usize, 0); }
            thread::sleep(Duration::from_millis(200));
            unsafe { PostMessageA(hwnd, WM_KEYDOWN, *hotkey as usize, 0); }
            thread::sleep(Duration::from_millis(100));
            unsafe { PostMessageA(hwnd, WM_KEYUP, *hotkey as usize, 0); }
            thread::sleep(Duration::from_millis(200));
            unsafe { PostMessageA(hwnd, WM_KEYDOWN, *hotkey as usize, 0); }
            thread::sleep(Duration::from_millis(100));
            unsafe { PostMessageA(hwnd, WM_KEYUP, *hotkey as usize, 0); }
            thread::sleep(Duration::from_millis(100));
            
        }
        None => {
            warn!("Hotkey not found for: {}", key);
        }
    }
}