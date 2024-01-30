
use crate::utils::env::{HP_CURRENT, HP_MAX, AETHER, TARGET_CHECK, P_X, P_Y, P_Z};
use crate::utils::address::{get_double_value_from_pointer_chain, get_value_memory};

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

pub fn is_hp_below_half() -> bool {
    let hp_current = get_hp_current();
    let hp_max = get_hp_max();
    hp_current < (hp_max / 2.0)
}

pub fn get_target() -> bool {
    let value = get_value_memory(TARGET_CHECK);
    eprintln!("Target: {}", value);
    value != 0
}

pub fn get_coord() -> [i32; 3] {
    let c_x = get_value_memory(P_X);
    let c_y = get_value_memory(P_Y);
    let c_z = get_value_memory(P_Z);
    let c = [c_x, c_y, c_z];
    c
}


