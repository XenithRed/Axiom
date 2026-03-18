use crate::world::block::BlockState;

#[derive(Debug, Clone)]
pub enum Palette {
    Single(BlockState),
    Indirect { a: Vec<BlockState>, b: Vec<u64>, c: u8 },
    Direct(Vec<u64>),
}

const DIRECT_BITS: u8 = 15;
const SECTION_VOL: usize = 4096;

impl Palette {
    pub fn new_single(s: BlockState) -> Self { Self::Single(s) }

    pub fn new_indirect(bits: u8) -> Self {
        let per_long = 64 / bits as usize;
        let longs = (SECTION_VOL + per_long - 1) / per_long;
        Self::Indirect { a: Vec::new(), b: vec![0u64; longs], c: bits }
    }

    pub fn new_direct() -> Self {
        let per_long = 64 / DIRECT_BITS as usize;
        let longs = (SECTION_VOL + per_long - 1) / per_long;
        Self::Direct(vec![0u64; longs])
    }

    pub fn get(&self, idx: usize) -> BlockState {
        match self {
            Self::Single(s) => *s,
            Self::Indirect { a, b, c } => {
                let entry = get_bits(b, idx, *c as usize);
                a.get(entry as usize).copied().unwrap_or(BlockState::AIR)
            }
            Self::Direct(b) => {
                BlockState(get_bits(b, idx, DIRECT_BITS as usize) as u32)
            }
        }
    }

    pub fn set(&mut self, idx: usize, state: BlockState) {
        match self {
            Self::Single(s) => {
                if *s == state { return; }
                let mut next = Self::new_indirect(4);
                let entry = next.ensure_indirect_entry(*s);
                for i in 0..SECTION_VOL {
                    if let Self::Indirect { b, c, .. } = &mut next {
                        set_bits(b, i, *c as usize, entry as u64);
                    }
                }
                *self = next;
                self.set(idx, state);
            }
            Self::Indirect { a, b, c } => {
                let bits = *c as usize;
                let entry = if let Some(i) = a.iter().position(|&s| s == state) {
                    i
                } else {
                    let i = a.len();
                    a.push(state);
                    if (1usize << bits) <= i {
                        self.upgrade();
                        self.set(idx, state);
                        return;
                    }
                    i
                };
                if let Self::Indirect { b, c, .. } = self {
                    set_bits(b, idx, *c as usize, entry as u64);
                }
            }
            Self::Direct(b) => {
                set_bits(b, idx, DIRECT_BITS as usize, state.0 as u64);
            }
        }
    }

    fn ensure_indirect_entry(&mut self, state: BlockState) -> usize {
        if let Self::Indirect { a, .. } = self {
            if let Some(i) = a.iter().position(|&s| s == state) { return i; }
            let i = a.len(); a.push(state); i
        } else { 0 }
    }

    fn upgrade(&mut self) {
        let old_bits = if let Self::Indirect { c, .. } = self { *c } else { return };
        let new_bits = (old_bits + 1).min(DIRECT_BITS);
        let mut next = if new_bits >= DIRECT_BITS {
            Self::new_direct()
        } else {
            Self::new_indirect(new_bits)
        };
        for i in 0..SECTION_VOL {
            next.set(i, self.get(i));
        }
        *self = next;
    }
}

fn get_bits(data: &[u64], idx: usize, bits: usize) -> u64 {
    let per = 64 / bits;
    let long_idx = idx / per;
    let bit_idx  = (idx % per) * bits;
    let mask = (1u64 << bits) - 1;
    (data.get(long_idx).copied().unwrap_or(0) >> bit_idx) & mask
}

fn set_bits(data: &mut Vec<u64>, idx: usize, bits: usize, val: u64) {
    let per = 64 / bits;
    let long_idx = idx / per;
    let bit_idx  = (idx % per) * bits;
    let mask = (1u64 << bits) - 1;
    if long_idx < data.len() {
        data[long_idx] = (data[long_idx] & !(mask << bit_idx)) | ((val & mask) << bit_idx);
    }
}
