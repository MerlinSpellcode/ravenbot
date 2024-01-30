use winapi::shared::minwindef::{DWORD};

pub const PROCESS_NAME: &str = "ravendawn_dx";
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
pub const PROCESS_ID: DWORD = 60004;
pub const HP_CURRENT: [usize; 2] = [0x027C3BA0, 0xCE0];
pub const HP_MAX: [usize; 2] = [0x027C3BA0, 0xCE8];
pub const AETHER: [usize; 2] = [0x027C3BA0, 0xD60];
pub const TARGET_CHECK: [usize; 2] = [0x027C3BA0, 0x84C];
pub const P_X: [usize; 2] = [0x027C3BA0, 0x18];
pub const P_Y: [usize; 2] = [0x027C3BA0, 0x1C];
pub const P_Z: [usize; 2] = [0x027C3BA0, 0x20];

// pub const PATH_WALK: [[i32; 3]; 13] = [
//     [5373, 5555, 7],
//     [5363, 5563, 7],
//     [5362, 5567, 7],
//     [5360, 5568, 7],
//     [5364, 5572, 7],
//     [5366, 5572, 7],
//     [5368, 5577, 7],
//     [5371, 5581, 7],
//     [5378, 5581, 7],
//     [5382, 5572, 7],
//     [5384, 5564, 7],
//     [5377, 5559, 7],
//     [5373, 5555, 7],
// ];

pub const PATH_WALK: [[i32; 3]; 18] = [
    [4915, 5529, 5],
    [4919, 5526, 5],
    [4919, 5524, 5],
    [4920, 5521, 5],
    [4920, 5517, 5],
    [4917, 5517, 5],
    [4915, 5517, 4],
    [4917, 5517, 5],
    [4920, 5517, 5],
    [4920, 5512, 5],
    [4919, 5508, 5],
    [4919, 5504, 5],
    [4919, 5508, 5],
    [4920, 5512, 5],
    [4920, 5517, 5],
    [4920, 5523, 5],
    [4919, 5525, 5],
    [4916, 5528, 5],
];
// [5378, 5579, 7]
// pub const POINTER_HP: usize = 0x2766E419C20 + CE0;
pub const CDR_Q: u64 = 12;
pub const CDR_E: u64 = 45;
pub const CDR_1: u64 = 15;
pub const CDR_2: u64 = 60;
pub const CDR_3: u64 = 15;

// pub const HP_TOTAL: usize = 0x17533851348;