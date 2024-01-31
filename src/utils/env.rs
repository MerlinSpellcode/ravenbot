use winapi::shared::minwindef::{DWORD};

pub const PROCESS_NAME: &str = "ravendawn_dx";
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
// pub const VK_F2: u8 = 0x71;
pub const VK_F5: u8 = 0x74;
pub const VK_F6: u8 = 0x75;
pub const VK_TAB: u8 = 0x09;
// pub const VK_F7: u8 = 0x76;
// pub const VK_F8: u8 = 0x77;
pub const VK_F9: u8 = 0x78;
pub const VK_F10: u8 = 0x79;
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