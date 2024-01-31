extern crate winapi;
extern crate serde;
extern crate serde_json;

use serde::{Deserialize};
use std::fs;
use winapi::um::winuser::{EnumWindows, GetWindowThreadProcessId, IsWindowVisible};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use winapi::shared::minwindef::{LPARAM, BOOL, DWORD};
use winapi::shared::windef::HWND;
use std::ptr;

use ravenbot::utils::address::get_base_address;
use ravenbot::commands::combat_instance;
use ravenbot::commands::path_walker;

#[derive(Deserialize)]
struct Config {
    coordinates_file: String,
}

#[derive(Deserialize)]
struct Coordinates {
    coordinates: Vec<[i32; 3]>,
}


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
     // Ler o arquivo config.json
    let config_contents = fs::read_to_string("config/config.json")
    .expect("Erro ao ler o arquivo config.json");

    // Deserializar o config.json para obter o caminho do arquivo de coordenadas
    let config: Config = serde_json::from_str(&config_contents)
        .expect("Erro ao deserializar config.json");

    // Ler o arquivo de coordenadas
    let coordinates_contents = fs::read_to_string(&config.coordinates_file)
        .expect("Erro ao ler o arquivo de coordenadas");

    // Deserializar o arquivo de coordenadas
    let coordinates: Coordinates = serde_json::from_str(&coordinates_contents)
        .expect("Erro ao deserializar o arquivo de coordenadas");

    let (_base_address, process_id) = match get_base_address() {
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
        loop {
            for path in coordinates.coordinates.iter() {
                combat_instance(window_info.hwnd);
                println!("Para onde está indo: {:?}", path);
                path_walker(window_info.hwnd, *path);
            }
        }
    }
}