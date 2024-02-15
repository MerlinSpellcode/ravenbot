use lazy_static::lazy_static;
use std::collections::HashMap;
use winapi::shared::minwindef::DWORD;
use serde::{Serialize, Deserialize};


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

#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    pub hunts: Vec<Hunt>,
    pub combat: Combat, 
    pub skills: Skills,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Skills {
    pub basic: Vec<BasicS>,
    pub start: Vec<Skill>,
    pub combo: Vec<Skill>,
    pub defense_light: Vec<Skill>,
    pub defense_full: Vec<Skill>, 
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Hunt {
    pub name: String,
    pub route: Vec<[i32; 3]>,
}

#[derive(Serialize, Deserialize, Debug)]
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

#[derive(Serialize, Deserialize, Debug)]
pub struct BasicS {
    pub hotkey: String,
    pub mana: u32,
    pub name: String,
}

pub const PROCESS_NAME: &str = "ravendawn_dx";
pub const BOT_OWNER: &str = "Default string";

pub const VK_Q: u8 = 0x51;
pub const VK_R: u8 = 0x52;
pub const VK_E: u8 = 0x45;
pub const VK_W: u8 = 0x57;
pub const VK_A: u8 = 0x41;
pub const VK_S: u8 = 0x53;
pub const VK_D: u8 = 0x44;
pub const VK_1: u8 = 0x31;
pub const VK_2: u8 = 0x32;
pub const VK_3: u8 = 0x33;
pub const VK_Z: u8 = 0x5A;
pub const VK_X: u8 = 0x58;
pub const VK_C: u8 = 0x43;
pub const VK_V: u8 = 0x56;
pub const VK_4: u8 = 0x34;
pub const VK_F1: u8 = 0x70;
pub const VK_F2: u8 = 0x71;
pub const VK_F5: u8 = 0x74;
pub const VK_F6: u8 = 0x75;
pub const VK_TAB: u8 = 0x09;
pub const VK_F7: u8 = 0x76;
pub const VK_F8: u8 = 0x77;
pub const VK_F9: u8 = 0x78;
pub const VK_F10: u8 = 0x79;
pub const VK_F11: u8 = 0x7A;
pub const VK_F12: u8 = 0x7B;

pub const PROCESS_ID: DWORD = 60004;
pub const HP_CURRENT: [usize; 2] = [0x027C4BA0, 0xCE0];
pub const HP_MAX: [usize; 2] = [0x027C4BA0, 0xCE8];
pub const MANA_MAX: [usize; 2] = [0x027C4BA0, 0xD30];
pub const MANA_CURRENT: [usize; 2] = [0x027C4BA0, 0xD28];
pub const AETHER: [usize; 2] = [0x027C4BA0, 0xD60];
pub const TARGET_CHECK: [usize; 2] = [0x027C4BA0, 0x84C];
pub const P_X: [usize; 2] = [0x027C4BA0, 0x18];
pub const P_Y: [usize; 2] = [0x027C4BA0, 0x1C];
pub const P_Z: [usize; 2] = [0x027C4BA0, 0x20];

// pub const PATH_WALK: [[i32; 3]; 19] = [
//     [4915, 5529, 5],
//     [4919, 5526, 5],
//     [4919, 5524, 5],
//     [4920, 5521, 5],
//     [4920, 5517, 5],
//     [4917, 5517, 5],
//     [4915, 5517, 4],
//     [4914, 5517, 4],
//     [4917, 5517, 5],
//     [4920, 5517, 5],
//     [4920, 5512, 5],
//     [4919, 5508, 5],
//     [4919, 5504, 5],
//     [4919, 5508, 5],
//     [4920, 5512, 5],
//     [4920, 5517, 5],
//     [4920, 5523, 5],
//     [4919, 5525, 5],
//     [4916, 5528, 5],
// ];

pub const PATH_WALK: [[i32; 3]; 27] = [
    [5373,5585,7],[5366,5593,7],[5364,5597,7],[5350,5599,7],[5345,5598,7],[5340,5597,7],[5337,5595,7],[5331,5595,7],[5327,5591,7],[5327,5588,7],[5327,5584,7],[5327,5578,7],[5326,5574,7],[5330,5567,7],[5337,5567,7],[5337,5575,7],[5341,5578,7],[5343,5586,7],[5345,5590,7],[5346,5593,7],[5349,5593,7],[5353,5593,7],[5360,5593,7],[5363,5593,7],[5368,5593,7],[5369,5591,7],[5369,5588,7]
];


// [5378, 5579, 7]s
// pub const POINTER_HP: usize = 0x2766E419C20 + CE0;
pub const CDR_COMBO_1: u64 = 15;
pub const CDR_COMBO_2: u64 = 30;


// pub const HP_TOTAL: usize = 0x17533851348;