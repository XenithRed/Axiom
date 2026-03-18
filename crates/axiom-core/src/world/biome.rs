use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default, Serialize, Deserialize)]
pub struct Biome(pub u32);

impl Biome {
    pub const PLAINS:  Self = Self(1);
    pub const DESERT:  Self = Self(2);
    pub const FOREST:  Self = Self(4);
    pub const OCEAN:   Self = Self(0);
    pub const NETHER:  Self = Self(8);
    pub const END:     Self = Self(9);

    pub fn id(self) -> u32 { self.0 }
}

#[derive(Debug, Clone)]
pub struct BiomePalette {
    a: Vec<u64>,
}

const BIOME_VOL: usize = 64;
const BIOME_BITS: usize = 6;

impl BiomePalette {
    pub fn new_uniform(b: Biome) -> Self {
        let per = 64 / BIOME_BITS;
        let mut data = vec![0u64; (BIOME_VOL + per - 1) / per];
        for i in 0..BIOME_VOL {
            let li = i / per;
            let bi = (i % per) * BIOME_BITS;
            data[li] |= (b.0 as u64 & 0x3F) << bi;
        }
        Self { a: data }
    }

    pub fn get(&self, idx: usize) -> Biome {
        let per = 64 / BIOME_BITS;
        let li  = idx / per;
        let bi  = (idx % per) * BIOME_BITS;
        let v   = (self.a.get(li).copied().unwrap_or(0) >> bi) & 0x3F;
        Biome(v as u32)
    }

    pub fn set(&mut self, idx: usize, b: Biome) {
        let per = 64 / BIOME_BITS;
        let li  = idx / per;
        let bi  = (idx % per) * BIOME_BITS;
        if li < self.a.len() {
            self.a[li] = (self.a[li] & !(0x3Fu64 << bi)) | ((b.0 as u64 & 0x3F) << bi);
        }
    }

    pub fn raw(&self) -> &[u64] { &self.a }
}

impl Default for BiomePalette {
    fn default() -> Self { Self::new_uniform(Biome::PLAINS) }
}
