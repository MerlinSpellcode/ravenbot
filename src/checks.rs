
use crate::utils::env::{BOT_OWNER, MANA_CURRENT, MANA_MAX, HP_CURRENT, HP_MAX, AETHER, TARGET_CHECK, P_X, P_Y, P_Z};
use crate::utils::address::{get_double_value_from_pointer_chain, get_value_memory};
use std::process::Command;

pub fn get_aether() -> f64 {
    let value = get_double_value_from_pointer_chain(&AETHER);
    value
}

pub fn get_hp_current() -> f64 {
    let value = get_double_value_from_pointer_chain(&HP_CURRENT);
    value
}

pub fn get_hp_max() -> f64 {
    let value = get_double_value_from_pointer_chain(&HP_MAX);
    value
}

pub fn get_mana_max() -> f64 {
    let value = get_double_value_from_pointer_chain(&MANA_MAX);
    value
}

pub fn get_mana_current() -> f64 {
    let value = get_double_value_from_pointer_chain(&MANA_CURRENT);
    value
}

pub fn mana_need_restore(value_percent: &str) -> bool {
    let mana_current = get_mana_current();
    let mana_max = get_mana_max();
    let percent_value = value_percent.trim_end_matches('%').parse::<f64>()
        .expect("Erro ao converter a porcentagem");
    let mana_threshold = mana_max * (percent_value / 100.0);
    mana_current < mana_threshold
}

pub fn hp_need_restore(value_percent: &str) -> bool {
    let hp_current = get_hp_current();
    let hp_max = get_hp_max();
    let percent_value = value_percent.trim_end_matches('%').parse::<f64>()
        .expect("Erro ao converter a porcentagem");
    let hp_threshold = hp_max * (percent_value / 100.0);
    hp_current < hp_threshold
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
    let value = get_value_memory(TARGET_CHECK);
    // eprintln!("Target: {}", value);
    value != 0
}

pub fn get_coord() -> [i32; 3] {
    let c_x = get_value_memory(P_X);
    let c_y = get_value_memory(P_Y);
    let c_z = get_value_memory(P_Z);
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
                return line.trim() == BOT_OWNER;
            }
        }
    }

    false
}