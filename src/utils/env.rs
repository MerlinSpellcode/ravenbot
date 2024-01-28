use winapi::shared::minwindef::{DWORD};

pub const VK_Q: u8 = 0x51;
pub const VK_E: u8 = 0x45;
pub const VK_W: u8 = 0x57;
pub const VK_A: u8 = 0x41;
pub const VK_S: u8 = 0x53;
pub const VK_D: u8 = 0x44;
pub const VK_1: u8 = 0x31;
pub const VK_2: u8 = 0x32;
pub const VK_3: u8 = 0x33;
// pub const VK_4: u8 = 0x34;
// pub const VK_F1: u8 = 0x70;
// pub const VK_F2: u8 = 0x71;
pub const VK_F5: u8 = 0x74;
pub const VK_F6: u8 = 0x75;
pub const VK_TAB: u8 = 0x09;
// pub const VK_F7: u8 = 0x76;
// pub const VK_F8: u8 = 0x77;
// pub const VK_F9: u8 = 0x78;
// pub const VK_F10: u8 = 0x79;
// pub const VK_F11: u8 = 0x7A;
// pub const VK_F12: u8 = 0x7B;
pub const PROCESS_ID: DWORD = 9920;
pub const HP_CURRENT: usize = 0x2766E422D0C;
pub const AETHER_CURRENT: usize = 0x2766E419AF0;
pub const MOVE_1: [&str; 1] = [
    "d",
];
pub const MOVE_2: [&str; 11] = [
    "d","d","d","d","d","d","d","d","d","d","d",
];
pub const MOVE_3: [&str; 12] = [
    "a","a","a","a","a","a","a","a","a","a","a","a",
];
// pub const POINTER_HP: usize = 0x2766E419C20 + CE0;
pub const CDR_Q: u64 = 12;
pub const CDR_E: u64 = 45;
pub const CDR_1: u64 = 15;
pub const CDR_2: u64 = 60;
pub const CDR_3: u64 = 15;

// pub const HP_TOTAL: usize = 0x17533851348;