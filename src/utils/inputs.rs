use winapi::um::winuser::{PostMessageA, WM_KEYDOWN, WM_KEYUP};
use winapi::shared::windef::HWND;
use std::thread;
use std::time::Duration;
use crate::utils::env::{VK_W, VK_A, VK_S, VK_D, VK_F5, VK_E};

pub fn press_w(hwnd: HWND) {
    unsafe { PostMessageA(hwnd, WM_KEYDOWN, VK_W as usize, 0); }
    unsafe { PostMessageA(hwnd, WM_KEYUP, VK_W as usize, 0); }
    thread::sleep(Duration::from_millis(400));
}
pub fn press_a(hwnd: HWND) {
    unsafe { PostMessageA(hwnd, WM_KEYDOWN, VK_A as usize, 0); }
    unsafe { PostMessageA(hwnd, WM_KEYUP, VK_A as usize, 0); }
    thread::sleep(Duration::from_millis(400));
}
pub fn press_s(hwnd: HWND) {
    unsafe { PostMessageA(hwnd, WM_KEYDOWN, VK_S as usize, 0); }
    unsafe { PostMessageA(hwnd, WM_KEYUP, VK_S as usize, 0); }
    thread::sleep(Duration::from_millis(400));
}
pub fn press_d(hwnd: HWND) {
    unsafe { PostMessageA(hwnd, WM_KEYDOWN, VK_D as usize, 0); }
    unsafe { PostMessageA(hwnd, WM_KEYUP, VK_D as usize, 0); }
    thread::sleep(Duration::from_millis(400));
}
pub fn press_f5(hwnd: HWND) {
    unsafe { PostMessageA(hwnd, WM_KEYDOWN, VK_F5 as usize, 0); }
    unsafe { PostMessageA(hwnd, WM_KEYUP, VK_F5 as usize, 0); }
    thread::sleep(Duration::from_millis(400));
}
pub fn press_e(hwnd: HWND) {
    unsafe { PostMessageA(hwnd, WM_KEYDOWN, VK_E as usize, 0); }
    unsafe { PostMessageA(hwnd, WM_KEYUP, VK_E as usize, 0); }
    thread::sleep(Duration::from_millis(400));
}