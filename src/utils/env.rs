use lazy_static::lazy_static;
use std::collections::HashMap;
use serde::{Serialize, Deserialize};
use std::fs::File;

use std::io::Read;
use base64::decode;
use serde_json;
// use std::str::FromStr;

lazy_static! {
    pub static ref HOTKEYS: HashMap<&'static str, u8> = {
        let mut m = HashMap::new();
        m.insert("F1", 0x70);
        m.insert("F2", 0x71);
        m.insert("F3", 0x72);
        m.insert("F4", 0x73);
        m.insert("F5", 0x74);
        m.insert("F6", 0x75);
        m.insert("F7", 0x76);
        m.insert("F8", 0x77);
        m.insert("F9", 0x78);
        m.insert("F10", 0x79);
        m.insert("F11", 0x7A);
        m.insert("F12", 0x7B);
        m.insert("A", 0x41);
        m.insert("B", 0x42);
        m.insert("C", 0x43);
        m.insert("D", 0x44);
        m.insert("E", 0x45);
        m.insert("F", 0x46);
        m.insert("G", 0x47);
        m.insert("H", 0x48);
        m.insert("I", 0x49);
        m.insert("J", 0x4A);
        m.insert("K", 0x4B);
        m.insert("L", 0x4C);
        m.insert("M", 0x4D);
        m.insert("N", 0x4E);
        m.insert("O", 0x4F);
        m.insert("P", 0x50);
        m.insert("Q", 0x51);
        m.insert("R", 0x52);
        m.insert("S", 0x53);
        m.insert("T", 0x54);
        m.insert("U", 0x55);
        m.insert("V", 0x56);
        m.insert("W", 0x57);
        m.insert("X", 0x58);
        m.insert("Y", 0x59);
        m.insert("Z", 0x5A);
        m.insert("0", 0x30);
        m.insert("1", 0x31);
        m.insert("2", 0x32);
        m.insert("3", 0x33);
        m.insert("4", 0x34);
        m.insert("5", 0x35);
        m.insert("6", 0x36);
        m.insert("7", 0x37);
        m.insert("8", 0x38);
        m.insert("9", 0x39);
        m
    };
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Config {
    pub hunts: Vec<Hunt>,
    pub combat: Combat, 
    pub skills: Skills,
    pub foods: Foods
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Foods {
    pub status: String,
    pub hp_mana_regen: String,
    pub attack_power: String,
    pub timer: u64,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Skills {
    pub basic: Vec<BasicS>,
    pub start: Vec<Skill>,
    pub combo: Vec<Skill>,
    pub defense_light: Vec<Skill>,
    pub defense_full: Vec<Skill>, 
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Hunt {
    pub name: String,
    pub route: Vec<[i32; 3]>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Combat {
    pub hp_regen_passive: String,
    pub mana_regen_passive: String,
    pub hp_to_defense_light: String,
    pub hp_to_defense_full: String,
    pub global_cd: u64,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Skill {
    pub hotkey: String,
    pub mana: u32,
    pub aether: bool,
    pub cooldown: u64,
    pub name: String,
    pub is_area: bool,
    pub prereq: String,
    pub has_global: bool
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Food {
    pub hotkey: String,
    pub duration: u64,
    pub name: String,
}

pub struct Buffs {
    pub food: Vec<Food>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct BasicS {
    pub hotkey: String,
    pub mana: u32,
    pub name: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct HardwareInfo {
    pub name: String,
    pub hwid: String,
    pub memory_path: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct EncodedHardwareInfo {
    key: String,
}

pub fn decode_hwid() -> String {
    let file_path = "key.json";
    let mut file = match File::open(file_path) {
        Ok(file) => file,
        Err(_) => panic!("Não foi possível abrir o arquivo {}", file_path),
    };
    
    let mut contents = String::new();
    if let Err(_) = file.read_to_string(&mut contents) {
        panic!("Falha ao ler o arquivo {}", file_path);
    }
    
    let encoded: EncodedHardwareInfo = match serde_json::from_str(&contents) {
        Ok(encoded) => encoded,
        Err(_) => panic!("Falha ao decodificar JSON"),
    };
    
    let decoded_bytes = match decode(&encoded.key) {
        Ok(bytes) => bytes,
        Err(_) => panic!("Falha ao decodificar Base64"),
    };
    
    let decoded_str = match String::from_utf8(decoded_bytes) {
        Ok(str) => str,
        Err(_) => panic!("Falha ao converter bytes decodificados para String"),
    };
    
    let hw_info: HardwareInfo = match serde_json::from_str(&decoded_str) {
        Ok(info) => info,
        Err(_) => panic!("Falha ao deserializar informações de hardware"),
    };

    hw_info.hwid
}

// Decodifica o memory path e retorna como usize
pub fn decode_mempath() -> usize {
    let file_path = "key.json";
    let mut file = match File::open(file_path) {
        Ok(file) => file,
        Err(_) => panic!("Não foi possível abrir o arquivo {}", file_path),
    };
    
    let mut contents = String::new();
    if let Err(_) = file.read_to_string(&mut contents) {
        panic!("Falha ao ler o arquivo {}", file_path);
    }
    
    let encoded: EncodedHardwareInfo = match serde_json::from_str(&contents) {
        Ok(encoded) => encoded,
        Err(_) => panic!("Falha ao decodificar JSON"),
    };
    
    let decoded_bytes = match decode(&encoded.key) {
        Ok(bytes) => bytes,
        Err(_) => panic!("Falha ao decodificar Base64"),
    };
    
    let decoded_str = match String::from_utf8(decoded_bytes) {
        Ok(str) => str,
        Err(_) => panic!("Falha ao converter bytes decodificados para String"),
    };
    
    let hw_info: HardwareInfo = match serde_json::from_str(&decoded_str) {
        Ok(info) => info,
        Err(_) => panic!("Falha ao deserializar informações de hardware"),
    };

    let mempath_str = hw_info.memory_path.trim_start_matches("0x");
    match usize::from_str_radix(mempath_str, 16) {
        Ok(num) => num,
        Err(_) => panic!("Falha ao converter memory_path para usize"),
    }
}


// pub const BOT_OWNER: &str = "07D7622_MB1E020747";
pub const PROCESS_NAME: &str = "ravendawn_dx";

// pub const HP_CURRENT: [usize; 2] = [0x027C6BC0, 0xCE0];
// pub const HP_MAX: [usize; 2] = [0x027C6BC0, 0xCE8];
// pub const MANA_MAX: [usize; 2] = [0x027C6BC0, 0xD30];
// pub const MANA_CURRENT: [usize; 2] = [0x027C6BC0, 0xD28];
// pub const AETHER: [usize; 2] = [0x027C6BC0, 0xD60];
// pub const TARGET_CHECK: [usize; 2] = [0x027C6BC0, 0x84C];
// pub const P_X: [usize; 2] = [0x027C6BC0, 0x18];
// pub const P_Y: [usize; 2] = [0x027C6BC0, 0x1C];
// pub const P_Z: [usize; 2] = [0x027C6BC0, 0x20];



pub const HP_CURRENT: usize = 0xCE0;
pub const HP_MAX: usize = 0xCE8;
pub const MANA_MAX: usize = 0xD30;
pub const MANA_CURRENT: usize = 0xD28;
pub const AETHER: usize = 0xD60;
pub const TARGET_CHECK: usize = 0x84C;
pub const P_X: usize = 0x18;
pub const P_Y: usize = 0x1C;
pub const P_Z: usize = 0x20;

pub const VK_TAB: u8 = 0x09;
pub const VK_W: u8 = 0x57;
pub const VK_A: u8 = 0x41;
pub const VK_S: u8 = 0x53;
pub const VK_D: u8 = 0x44;
