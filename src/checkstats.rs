use winapi::um::winnt::PROCESS_ALL_ACCESS;
use winapi::um::memoryapi::ReadProcessMemory;
use winapi::um::processthreadsapi::OpenProcess;
use std::{ptr::null_mut, mem};
use crate::utils::env::{AETHER_CURRENT, HP_CURRENT, PROCESS_ID}; // Importando as constantes VK_F5 e VK_E do arquivo env.rs

pub fn check_aether() -> i32 {
    let process_handle = unsafe { OpenProcess(PROCESS_ALL_ACCESS, 0, PROCESS_ID) };

    if process_handle.is_null() {
        eprintln!("Erro ao abrir o processo");
        return -1;
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

    let return_value = if result == 0 {
        eprintln!("Erro ao ler a memória do Aether");
        -1
    } else {
        value as i32
    };

    unsafe { winapi::um::handleapi::CloseHandle(process_handle) };
    return_value
}


pub fn check_hp() -> i32 {
    let process_handle = unsafe { OpenProcess(PROCESS_ALL_ACCESS, 0, PROCESS_ID) };

    if process_handle.is_null() {
        eprintln!("Erro ao abrir o processo");
        return -1;
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

    let return_value = if result == 0 {
        eprintln!("Erro ao ler a memória do HP");
        -1
    } else {
        value as i32
    };

    unsafe { winapi::um::handleapi::CloseHandle(process_handle) };
    return_value
}