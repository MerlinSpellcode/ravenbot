use winapi::um::winnt::PROCESS_ALL_ACCESS;
use winapi::um::memoryapi::ReadProcessMemory;
use winapi::um::processthreadsapi::OpenProcess;
use std::{ptr::null_mut, mem};
use crate::utils::env::{AETHER_CURRENT, HP_CURRENT, PROCESS_ID}; // Importando as constantes VK_F5 e VK_E do arquivo env.rs

pub fn check_aether(){
    let process_handle = unsafe { OpenProcess(PROCESS_ALL_ACCESS, 0, PROCESS_ID) };

    if process_handle.is_null() {
        eprintln!("Erro ao abrir o processo");
        return;
    }
    let mut value: f64 = 0.0;
    let result = unsafe {
        ReadProcessMemory(
            process_handle,
            AETHER_CURRENT as *const _,
            &mut value as *mut _ as *mut _,
            mem::size_of::<f64>(),
            null_mut(),
        )
    };
    if result == 0 {
        eprintln!("Erro ao ler a memória do Aether");
    } else {
        // return value;
        println!("AETHER: {}", value);
    }
    unsafe { winapi::um::handleapi::CloseHandle(process_handle) };
}

pub fn check_hp(){
    let process_handle = unsafe { OpenProcess(PROCESS_ALL_ACCESS, 0, PROCESS_ID) };

    if process_handle.is_null() {
        eprintln!("Erro ao abrir o processo");
        return;
    }
    let mut value: f64 = 0.0;
    let result = unsafe {
        ReadProcessMemory(
            process_handle,
            HP_CURRENT as *const _,
            &mut value as *mut _ as *mut _,
            mem::size_of::<f64>(),
            null_mut(),
        )
    };
    if result == 0 {
        eprintln!("Erro ao ler a memória do HP");
    } else {
        // return value;
        println!("HP: {}", value);
    }
    unsafe { winapi::um::handleapi::CloseHandle(process_handle) };
}