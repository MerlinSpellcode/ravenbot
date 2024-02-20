
use log::info;

use crate::utils::env::{decode_hwid, decode_mempath, MANA_CURRENT, MANA_MAX, HP_CURRENT, HP_MAX, AETHER, TARGET_CHECK, P_X, P_Y, P_Z};
use crate::utils::address::{get_double_value_from_pointer_chain, get_value_memory};
use std::process::Command;

pub fn get_aether() -> f64 {
    let mempath_info = decode_mempath();
    let mempath_pointer: [usize; 2] = [mempath_info, AETHER];
    let value = get_double_value_from_pointer_chain(&mempath_pointer);
    value
}

pub fn get_hp_current() -> f64 {
    let mempath_info = decode_mempath();
    let mempath_pointer: [usize; 2] = [mempath_info, HP_CURRENT];
    let value = get_double_value_from_pointer_chain(&mempath_pointer);
    value
}


pub fn get_hp_max() -> f64 {
    let mempath_info = decode_mempath();
    let mempath_pointer: [usize; 2] = [mempath_info, HP_MAX];
    let value = get_double_value_from_pointer_chain(&mempath_pointer);
    value
}

pub fn get_mana_max() -> f64 {
    let mempath_info = decode_mempath();
    let mempath_pointer: [usize; 2] = [mempath_info, MANA_MAX];
    let value = get_double_value_from_pointer_chain(&mempath_pointer);
    value
}

pub fn get_mana_current() -> f64 {
    let mempath_info = decode_mempath();
    let mempath_pointer: [usize; 2] = [mempath_info, MANA_CURRENT];
    let value = get_double_value_from_pointer_chain(&mempath_pointer);
    value
}

pub fn get_mana_actual() {
    let mana_current = get_mana_current();
    let mana_max = get_mana_max();
    info!("Mana: {}/{}", mana_current, mana_max);
}

pub fn get_hp_actual() {
    let hp_current = get_hp_current();
    let hp_max = get_hp_max();
    info!("HP: {}/{}", hp_current, hp_max);
}

// pub fn mana_need_restore(value_percent: &str) -> bool {
//     let mana_current = get_mana_current();
//     let mana_max = get_mana_max();
//     let percent_value = value_percent.trim_end_matches('%').parse::<f64>()
//         .expect("Erro ao converter a porcentagem");
//     let mana_threshold = mana_max * (percent_value / 100.0);
//     mana_current < mana_threshold
// }

pub fn hp_need_combat_restore(hp_to_combat_restore: &str) -> bool {
    let hp_current = get_hp_current(); // ex.: 1500
    let hp_max = get_hp_max(); // ex.: 2000
    let percent_value = hp_to_combat_restore.trim_end_matches('%').parse::<f64>()
        .expect("Erro ao converter a porcentagem de hp_to_combat_restore"); // 70.0
    let hp_threshold = hp_max * (percent_value / 100.0); // 2000 * (70.0 / 100) = 1400 
    hp_current < hp_threshold // 1000 < 1400
}

pub fn hp_need_passive_restore(hp_to_regen_passive: &str) -> bool {
    let hp_current = get_hp_current(); // ex.:1500
    let hp_max = get_hp_max(); // ex.: 4250
    let hp_to_regen_passive_percent_value = hp_to_regen_passive.trim_end_matches('%').parse::<f64>()
        .expect("Erro ao converter a porcentagem de hp_to_regen_passive"); // 90.0
    let hp_threshold = hp_max * (hp_to_regen_passive_percent_value / 100.0); // 4250 * (90.0 / 100) = 2550
    hp_current < hp_threshold // 1750 < 3825
}

pub fn mana_need_passive_restore(mana_to_regen_passive: &str) -> bool {
    let mana_current = get_mana_current(); // ex.: 1500
    let mana_max = get_mana_max(); // ex.: 2000
    let mana_to_regen_passive_percent_value = mana_to_regen_passive.trim_end_matches('%').parse::<f64>()
        .expect("Erro ao converter a porcentagem de mana_to_regen_passive"); // 70.0
    let mana_threshold = mana_max * (mana_to_regen_passive_percent_value / 100.0); // 2000 * (70.0 / 80) = 1750
    mana_current < mana_threshold // 1750 < 1750
}

pub fn hp_can_continue(hp_to_continue: &str) -> bool {
    let hp_current = get_hp_current(); // ex.: 3000
    let hp_max = get_hp_max(); // ex.: 4250
    let hp_to_continue_percent_value = hp_to_continue.trim_end_matches('%').parse::<f64>()
        .expect("Erro ao converter a porcentagem de mana_to_regen_passive"); // 80.0
    let hp_threshold = hp_max * (hp_to_continue_percent_value / 100.0); // 4250 * (80.0 / 100) = 3400
    hp_current < hp_threshold // 3000 < 3400 = false
}

pub fn mana_can_continue(mana_to_continue: &str) -> bool {
    let mana_current = get_mana_current(); // ex.: 1500
    let mana_max = get_mana_max();
    let mana_to_continue_percent_value = mana_to_continue.trim_end_matches('%').parse::<f64>()
        .expect("Erro ao converter a porcentagem de mana_to_regen_passive"); // 70.0
    let mana_threshold = mana_max * (mana_to_continue_percent_value / 100.0); // 2000 * (70.0 / 80) = 1750
    mana_current < mana_threshold
}

pub fn hp_need_drink(value_percent: &str) -> bool {
    let hp_current = get_hp_current();
    let hp_max = get_hp_max();
    let percent_value = value_percent.trim_end_matches('%').parse::<f64>()
        .expect("Erro ao converter a porcentagem");
    let hp_threshold = hp_max * (percent_value / 100.0);
    hp_current < hp_threshold
}

pub fn mana_need_drink(value_percent: &str) -> bool {
    let mana_current = get_mana_current();
    let mana_max = get_mana_max();
    let percent_value = value_percent.trim_end_matches('%').parse::<f64>()
        .expect("Erro ao converter a porcentagem");
    let mana_threshold = mana_max * (percent_value / 100.0);
    mana_current < mana_threshold
}


pub fn is_hp_full() -> bool {
    let hp_current = get_hp_current();
    let hp_max = get_hp_max();
    hp_current != hp_max
}

pub fn is_mana_full() -> bool {
    let mana_current = get_mana_current();
    let mana_max = get_mana_max();
    mana_current != mana_max
}

pub fn get_target() -> bool {
    let mempath_info = decode_mempath();
    let mempath_pointer: [usize; 2] = [mempath_info, TARGET_CHECK];
    let value = get_value_memory(mempath_pointer);
    value != 0
}

pub fn get_coord() -> [i32; 3] {
    let mempath_info = decode_mempath();
    let m_x: [usize; 2] = [mempath_info, P_X];
    let m_y: [usize; 2] = [mempath_info, P_Y];
    let m_z: [usize; 2] = [mempath_info, P_Z];
    let c_x = get_value_memory(m_x);
    let c_y = get_value_memory(m_y);
    let c_z = get_value_memory(m_z);
    let c = [c_x, c_y, c_z];
    c
}

pub fn check_hwid() -> bool {
    let output = Command::new("wmic")
        .args(["baseboard", "get", "SerialNumber"])
        .output()
        .expect("Falha ao executar o comando WMIC");

    if output.status.success() {
        let raw_output = String::from_utf8_lossy(&output.stdout);
        for line in raw_output.lines() {
            if !line.starts_with("SerialNumber") {
                let hw_info = decode_hwid(); // Isso pode falhar, então estamos propagando o erro com `?`
                return line.trim() == hw_info;
            }
        }
    }

    false
}