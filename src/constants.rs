/// no of cards in a pack minus any jokers
pub const PACK_SIZE: usize = 52;
pub const PACK_LO: usize = 0;
pub const PACK_Q1: usize = PACK_SIZE / 4;
pub const PACK_Q2: usize = PACK_SIZE / 2;
pub const PACK_Q3: usize = (PACK_SIZE / 2) + (PACK_SIZE / 4);
// repeat for u8
pub const PACK_SIZE_U8: u8 = 52;
pub const PACK_LO_U8: u8 = 0;
pub const PACK_Q1_U8: u8 = PACK_SIZE_U8 / 4;
pub const PACK_Q2_U8: u8 = PACK_SIZE_U8 / 2;
pub const PACK_Q3_U8: u8 = (PACK_SIZE_U8 / 2) + (PACK_SIZE_U8 / 4);
