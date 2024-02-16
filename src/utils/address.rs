use log::error;
use winapi::um::processthreadsapi::OpenProcess;
use winapi::um::psapi::{EnumProcessModules, GetModuleInformation, MODULEINFO};
use winapi::um::winnt::PROCESS_ALL_ACCESS;
use winapi::shared::minwindef::{DWORD, HMODULE};
use winapi::um::tlhelp32::{CreateToolhelp32Snapshot, Process32First, Process32Next, PROCESSENTRY32, TH32CS_SNAPPROCESS};
use winapi::um::handleapi::CloseHandle;
use winapi::um::memoryapi::ReadProcessMemory;
use std::{ptr::null_mut, mem};
use std::ffi::CStr;
use crate::utils::env::PROCESS_NAME;

pub fn get_base_address() -> Option<(usize, DWORD)> {
    // Busca pelo processo
    let process_id = {
        let snapshot = unsafe { CreateToolhelp32Snapshot(TH32CS_SNAPPROCESS, 0) };
        if snapshot.is_null() {
            return None;
        }

        let mut process_entry = PROCESSENTRY32 {
            dwSize: mem::size_of::<PROCESSENTRY32>() as u32,
            cntUsage: 0,
            th32ProcessID: 0,
            th32DefaultHeapID: 0,
            th32ModuleID: 0,
            cntThreads: 0,
            th32ParentProcessID: 0,
            pcPriClassBase: 0,
            dwFlags: 0,
            szExeFile: [0; winapi::shared::minwindef::MAX_PATH],
        };
        let mut found: Option<DWORD> = None;

        if unsafe { Process32First(snapshot, &mut process_entry) } != 0 {
            loop {
                let process_name = unsafe { CStr::from_ptr(process_entry.szExeFile.as_ptr()).to_string_lossy() };
                if process_name.contains(PROCESS_NAME) {
                    found = Some(process_entry.th32ProcessID);
                    break;
                }

                if unsafe { Process32Next(snapshot, &mut process_entry) } == 0 {
                    break;
                }
            }
        }

        unsafe { CloseHandle(snapshot) };
        found
    };

    let process_id = process_id?;

    // Abre o processo
    let process_handle = unsafe { OpenProcess(PROCESS_ALL_ACCESS, 0, process_id) };
    if process_handle.is_null() {
        return None;
    }

    let mut h_mods: [HMODULE; 1024] = [0 as HMODULE; 1024];
    let mut cb_needed: DWORD = 0;

    if unsafe {
        EnumProcessModules(process_handle, h_mods.as_mut_ptr(), mem::size_of::<HMODULE>() as u32 * 1024, &mut cb_needed)
    } == 0 {
        return None;
    }

    for i in 0..(cb_needed / mem::size_of::<HMODULE>() as DWORD) {
        let mut mod_info = unsafe { mem::MaybeUninit::<MODULEINFO>::uninit().assume_init_read() };
        if unsafe { GetModuleInformation(process_handle, h_mods[i as usize], &mut mod_info, mem::size_of::<MODULEINFO>() as DWORD) } != 0 {
            return Some((mod_info.lpBaseOfDll as usize, process_id));
        }
    }

    None
}

pub fn get_value_memory(atr_get: [usize; 2]) -> i32 {
    // Encontra o endereço base do módulo
    let (base_address, process_id) = match get_base_address() {
        Some(data) => data,
        None => {
            error!("Erro ao encontrar o endereço base do módulo");
            return -1;
        }
    };

    let process_handle = unsafe { OpenProcess(PROCESS_ALL_ACCESS, 0, process_id) };
    if process_handle.is_null() {
        error!("Erro ao abrir o processo (HP)");
        return -1;
    }

    // Calcula o endereço do ponteiro
    let mut pointer_address = base_address + atr_get[0];
    unsafe {
        ReadProcessMemory(
            process_handle,
            pointer_address as *const _,
            &mut pointer_address as *mut _ as *mut _,
            mem::size_of::<usize>(),
            null_mut(),
        );
    }

    pointer_address += atr_get[1];

    let mut value: i32 = 0;
    let result = unsafe {
        ReadProcessMemory(
            process_handle,
            pointer_address as *const _,
            &mut value as *mut _ as *mut _,
            mem::size_of::<i32>(),
            null_mut(),
        )
    };

    let return_value = if result == 0 {
        error!("Erro ao ler a memória do HP");
        -1
    } else {
        value as i32
    };

    unsafe { winapi::um::handleapi::CloseHandle(process_handle) };
    return_value
}

pub fn get_double_value_from_pointer_chain(base_offsets: &[usize]) -> f64 {
    let (base_address, process_id) = match get_base_address() {
        Some(data) => data,
        None => {
            error!("Erro ao encontrar o endereço base do módulo");
            return -1.0;
        }
    };

    let process_handle = unsafe { OpenProcess(PROCESS_ALL_ACCESS, 0, process_id) };
    if process_handle.is_null() {
        error!("Erro ao abrir o processo");
        return -1.0;
    }

    let mut pointer_address = base_address;
    for &offset in base_offsets.iter().take(base_offsets.len() - 1) {
        pointer_address += offset;
        unsafe {
            ReadProcessMemory(
                process_handle,
                pointer_address as *const _,
                &mut pointer_address as *mut _ as *mut _,
                mem::size_of::<usize>(),
                null_mut(),
            );
        }
    }

    // Adiciona o último offset
    pointer_address += base_offsets.last().unwrap();

    let mut value: f64 = 0.0;
    let result = unsafe {
        ReadProcessMemory(
            process_handle,
            pointer_address as *const _,
            &mut value as *mut _ as *mut _,
            mem::size_of::<f64>(),
            null_mut(),
        )
    };

    let return_value = if result == 0 {
        error!("Erro ao ler a memória");
        -1.0
    } else {
        value
    };
    

    unsafe { winapi::um::handleapi::CloseHandle(process_handle) };
    return_value
}