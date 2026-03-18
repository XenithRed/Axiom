use super::{biome::BiomePalette, block::BlockState, palette::Palette};

const VOL: usize = 4096;

#[derive(Debug, Clone)]
pub struct Section {
    pub a: i32,
    pub b: Palette,
    pub c: BiomePalette,
    pub d: [u8; 2048],
    pub e: [u8; 2048],
    pub f: i32,
}

impl Section {
    pub fn new(y: i32) -> Self {
        Self {
            a: y,
            b: Palette::new_single(BlockState::AIR),
            c: BiomePalette::default(),
            d: [0xFF; 2048],
            e: [0u8; 2048],
            f: 0,
        }
    }

    pub fn get(&self, x: usize, y: usize, z: usize) -> BlockState {
        self.b.get(idx(x, y, z))
    }

    pub fn set(&mut self, x: usize, y: usize, z: usize, s: BlockState) {
        let old = self.get(x, y, z);
        if old == s { return; }
        if !old.is_air() { self.f -= 1; }
        if !s.is_air()   { self.f += 1; }
        self.b.set(idx(x, y, z), s);
    }

    pub fn sky_light(&self, x: usize, y: usize, z: usize) -> u8 {
        nibble(&self.d, idx(x, y, z))
    }

    pub fn block_light(&self, x: usize, y: usize, z: usize) -> u8 {
        nibble(&self.e, idx(x, y, z))
    }

    pub fn set_sky_light(&mut self, x: usize, y: usize, z: usize, v: u8) {
        set_nibble(&mut self.d, idx(x, y, z), v);
    }

    pub fn set_block_light(&mut self, x: usize, y: usize, z: usize, v: u8) {
        set_nibble(&mut self.e, idx(x, y, z), v);
    }

    pub fn non_air(&self) -> i32 { self.f }

    pub fn is_empty(&self) -> bool { self.f == 0 }
}

#[inline]
fn idx(x: usize, y: usize, z: usize) -> usize {
    (y << 8) | (z << 4) | x
}

#[inline]
fn nibble(arr: &[u8; 2048], idx: usize) -> u8 {
    let byte = arr[idx >> 1];
    if idx & 1 == 0 { byte & 0x0F } else { byte >> 4 }
}

#[inline]
fn set_nibble(arr: &mut [u8; 2048], idx: usize, v: u8) {
    let b = &mut arr[idx >> 1];
    if idx & 1 == 0 {
        *b = (*b & 0xF0) | (v & 0x0F);
    } else {
        *b = (*b & 0x0F) | ((v & 0x0F) << 4);
    }
}
