use bytes::{BufMut, Bytes, BytesMut};

pub const SECTION_LIGHT: usize = 2048;

pub fn java_sky_to_bedrock(sky: &[u8; 2048]) -> [u8; 2048] {
    *sky
}

pub fn java_block_to_bedrock(blk: &[u8; 2048]) -> [u8; 2048] {
    *blk
}

pub fn full_bright_sky() -> [u8; 2048] {
    [0xFF; 2048]
}

pub fn zero_light() -> [u8; 2048] {
    [0x00; 2048]
}

pub fn get_nibble(arr: &[u8; 2048], idx: usize) -> u8 {
    let byte = arr[idx >> 1];
    if idx & 1 == 0 { byte & 0x0F } else { byte >> 4 }
}

pub fn set_nibble(arr: &mut [u8; 2048], idx: usize, v: u8) {
    let b = &mut arr[idx >> 1];
    if idx & 1 == 0 {
        *b = (*b & 0xF0) | (v & 0x0F);
    } else {
        *b = (*b & 0x0F) | ((v & 0x0F) << 4);
    }
}
