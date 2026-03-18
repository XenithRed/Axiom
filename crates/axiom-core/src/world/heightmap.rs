use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum HmType {
    MotionBlocking,
    MotionBlockingNoLeaves,
    OceanFloor,
    WorldSurface,
}

#[derive(Debug, Clone)]
pub struct Heightmap {
    pub a: HmType,
    pub b: [i16; 256],
}

impl Heightmap {
    pub fn new(t: HmType) -> Self {
        Self { a: t, b: [0i16; 256] }
    }

    pub fn get(&self, x: usize, z: usize) -> i16 {
        self.b[z * 16 + x]
    }

    pub fn set(&mut self, x: usize, z: usize, y: i16) {
        self.b[z * 16 + x] = y;
    }

    pub fn packed_longs(&self) -> Vec<u64> {
        let bits = 9usize;
        let per  = 64 / bits;
        let mut out = vec![0u64; (256 + per - 1) / per];
        for (i, &h) in self.b.iter().enumerate() {
            let li = i / per;
            let bi = (i % per) * bits;
            out[li] |= (h as u64 & 0x1FF) << bi;
        }
        out
    }

    pub fn from_packed(t: HmType, longs: &[u64]) -> Self {
        let bits = 9usize;
        let per  = 64 / bits;
        let mut b = [0i16; 256];
        for i in 0..256 {
            let li = i / per;
            let bi = (i % per) * bits;
            let v  = (longs.get(li).copied().unwrap_or(0) >> bi) & 0x1FF;
            b[i] = v as i16;
        }
        Self { a: t, b }
    }
}
