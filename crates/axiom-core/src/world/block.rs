use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default, Serialize, Deserialize)]
pub struct BlockState(pub u32);

impl BlockState {
    pub const AIR: Self = Self(0);

    pub fn new(id: u32) -> Self { Self(id) }
    pub fn id(self) -> u32 { self.0 }
    pub fn is_air(self) -> bool { self.0 == 0 }
}

impl std::fmt::Display for BlockState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "BlockState({})", self.0)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct BlockPos {
    pub x: i32,
    pub y: i32,
    pub z: i32,
}

impl BlockPos {
    pub fn new(x: i32, y: i32, z: i32) -> Self { Self { x, y, z } }

    pub fn chunk_xz(self) -> (i32, i32) {
        (self.x >> 4, self.z >> 4)
    }

    pub fn section_y(self) -> i32 { self.y >> 4 }

    pub fn local(self) -> (usize, usize, usize) {
        ((self.x & 0xF) as usize, (self.y & 0xF) as usize, (self.z & 0xF) as usize)
    }

    pub fn neighbors(self) -> [BlockPos; 6] {
        let Self { x, y, z } = self;
        [
            Self::new(x+1, y, z), Self::new(x-1, y, z),
            Self::new(x, y+1, z), Self::new(x, y-1, z),
            Self::new(x, y, z+1), Self::new(x, y, z-1),
        ]
    }
}

impl From<(i32, i32, i32)> for BlockPos {
    fn from((x, y, z): (i32, i32, i32)) -> Self { Self { x, y, z } }
}
