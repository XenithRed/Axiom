use ahash::AHashMap;
use super::{
    block::{BlockPos, BlockState},
    heightmap::{Heightmap, HmType},
    section::Section,
};
use crate::error::{Err, R};

pub const MIN_Y: i32 = -64;
pub const MAX_Y: i32 = 320;
pub const SECTION_COUNT: usize = ((MAX_Y - MIN_Y) / 16) as usize;

#[derive(Debug, Clone)]
pub struct Chunk {
    pub a: i32,
    pub b: i32,
    pub c: AHashMap<i32, Section>,
    pub d: Heightmap,
    pub e: Heightmap,
    pub f: bool,
}

impl Chunk {
    pub fn new(cx: i32, cz: i32) -> Self {
        Self {
            a: cx,
            b: cz,
            c: AHashMap::new(),
            d: Heightmap::new(HmType::MotionBlocking),
            e: Heightmap::new(HmType::WorldSurface),
            f: false,
        }
    }

    pub fn get_block(&self, pos: BlockPos) -> BlockState {
        let sy = pos.section_y();
        let (lx, ly, lz) = pos.local();
        self.c.get(&sy).map(|s| s.get(lx, ly, lz)).unwrap_or(BlockState::AIR)
    }

    pub fn set_block(&mut self, pos: BlockPos, state: BlockState) -> R<()> {
        let y = pos.y;
        if y < MIN_Y || y >= MAX_Y { return Err(Err::OutOfRange(pos.x, y, pos.z)); }
        let sy = pos.section_y();
        let (lx, ly, lz) = pos.local();
        self.c.entry(sy).or_insert_with(|| Section::new(sy)).set(lx, ly, lz, state);
        self.update_heightmap(pos);
        Ok(())
    }

    pub fn section(&self, sy: i32) -> Option<&Section> { self.c.get(&sy) }

    pub fn section_mut(&mut self, sy: i32) -> &mut Section {
        self.c.entry(sy).or_insert_with(|| Section::new(sy))
    }

    pub fn sections(&self) -> impl Iterator<Item = &Section> {
        self.c.values()
    }

    pub fn height_mb(&self, x: usize, z: usize) -> i16 { self.d.get(x, z) }
    pub fn height_ws(&self, x: usize, z: usize) -> i16 { self.e.get(x, z) }

    fn update_heightmap(&mut self, pos: BlockPos) {
        let lx = (pos.x & 0xF) as usize;
        let lz = (pos.z & 0xF) as usize;
        let b  = self.get_block(pos);
        if !b.is_air() {
            let h = self.d.get(lx, lz);
            if pos.y >= h as i32 { self.d.set(lx, lz, (pos.y + 1) as i16); }
            let h = self.e.get(lx, lz);
            if pos.y >= h as i32 { self.e.set(lx, lz, (pos.y + 1) as i16); }
        }
    }
}

#[derive(Debug, Default)]
pub struct ChunkMap {
    a: AHashMap<(i32, i32), Chunk>,
}

impl ChunkMap {
    pub fn new() -> Self { Self::default() }

    pub fn load(&mut self, chunk: Chunk) {
        self.a.insert((chunk.a, chunk.b), chunk);
    }

    pub fn unload(&mut self, cx: i32, cz: i32) -> Option<Chunk> {
        self.a.remove(&(cx, cz))
    }

    pub fn get(&self, cx: i32, cz: i32) -> Option<&Chunk> {
        self.a.get(&(cx, cz))
    }

    pub fn get_mut(&mut self, cx: i32, cz: i32) -> Option<&mut Chunk> {
        self.a.get_mut(&(cx, cz))
    }

    pub fn get_block(&self, pos: BlockPos) -> BlockState {
        let (cx, cz) = pos.chunk_xz();
        self.get(cx, cz).map(|c| c.get_block(pos)).unwrap_or(BlockState::AIR)
    }

    pub fn set_block(&mut self, pos: BlockPos, state: BlockState) -> R<()> {
        let (cx, cz) = pos.chunk_xz();
        self.get_mut(cx, cz)
            .ok_or(Err::NoChunk(cx, cz))?
            .set_block(pos, state)
    }

    pub fn is_loaded(&self, cx: i32, cz: i32) -> bool {
        self.a.contains_key(&(cx, cz))
    }

    pub fn loaded_count(&self) -> usize { self.a.len() }

    pub fn iter(&self) -> impl Iterator<Item = &Chunk> { self.a.values() }
}
