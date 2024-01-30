extern crate winapi;

use winapi::um::winuser::{EnumWindows, GetWindowThreadProcessId, IsWindowVisible};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use winapi::shared::minwindef::{LPARAM, BOOL, DWORD};
use winapi::shared::windef::HWND;
use std::ptr;

use ravenbot::utils::env::PATH_WALK;
use ravenbot::utils::address::get_base_address;
use ravenbot::commands::combat_instance;
use ravenbot::commands::path_walker;
use ravenbot::commands::time_test;
use ravenbot::checks::is_hp_below_half;
use ravenbot::checks::get_aether;
use ravenbot::checks::get_target;
use ravenbot::checks::get_coord;

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
    
    let (base_address, process_id) = match get_base_address() {
        Some(data) => data,
        None => {
            eprintln!("Erro ao encontrar o endereço base do módulo");
            return;
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
        return;
    }

    let running = Arc::new(AtomicBool::new(true));
    let r = running.clone();

    ctrlc::set_handler(move || {
        r.store(false, Ordering::SeqCst);
    }).expect("Erro ao definir o manipulador de Ctrl-C");

    while running.load(Ordering::SeqCst) {
        // let current_value = get_coord();
        // println!("COORDENADAS: {:?}", current_value);
        // break;
        // time_test();

        loop {
            for path in PATH_WALK.iter() {
                combat_instance(window_info.hwnd);
                println!("Para onde está indo: {:?}", path);
                path_walker(window_info.hwnd, *path);
            }
        }
    }
}