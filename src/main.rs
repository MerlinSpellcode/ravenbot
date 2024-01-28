extern crate winapi;

use winapi::um::winuser::{EnumWindows, GetWindowThreadProcessId, IsWindowVisible};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use winapi::shared::minwindef::{LPARAM, BOOL, DWORD};
use winapi::shared::windef::HWND;
use std::ptr;

use ravenbot::utils::env::PROCESS_ID;
use ravenbot::utils::env::MOVE_1;
use ravenbot::utils::env::MOVE_2;
use ravenbot::utils::env::MOVE_3;
use ravenbot::commands::skill_rotation_1;
use ravenbot::commands::skill_rotation_2;
use ravenbot::commands::skill_rotation_3;
use ravenbot::commands::walk_hunt;
use ravenbot::checkstats::check_hp;

struct WindowInfo {
    game_p_id: DWORD,
    hwnd: HWND,
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


fn main() {
    let mut window_info = WindowInfo {
        game_p_id: PROCESS_ID,
        hwnd: ptr::null_mut(),
    };

    unsafe {
        EnumWindows(Some(enum_windows_callback), &mut window_info as *mut _ as LPARAM);
    }
    
    if window_info.hwnd.is_null() {
        eprintln!("Janela não encontrada");
        return;
    }

    check_hp();

    let running = Arc::new(AtomicBool::new(true));
    let r = running.clone();

    ctrlc::set_handler(move || {
        r.store(false, Ordering::SeqCst);
    }).expect("Erro ao definir o manipulador de Ctrl-C");

    while running.load(Ordering::SeqCst) {
        walk_hunt(window_info.hwnd, &MOVE_1);
        skill_rotation_1(window_info.hwnd);
        walk_hunt(window_info.hwnd, &MOVE_2);
        skill_rotation_2(window_info.hwnd);
        walk_hunt(window_info.hwnd, &MOVE_3);
        skill_rotation_3(window_info.hwnd);
    }
}
