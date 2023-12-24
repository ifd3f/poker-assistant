pub const ORDERED_HANDS_RAW: &[u8] = include_bytes!(concat!(env!("OUT_DIR"), "/ordered_hands.bin"));

pub const N_HANDS: usize = ORDERED_HANDS_RAW.len() / 4;
