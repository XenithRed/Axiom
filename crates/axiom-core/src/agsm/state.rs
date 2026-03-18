use ahash::AHashMap;
use uuid::Uuid;
use crate::{
    entity::{Ecs, Eid, Kind},
    player::{ConnState, GameMode, Player},
    world::{BlockPos, BlockState, ChunkMap},
};

#[derive(Debug)]
pub struct Agsm {
    pub a: ChunkMap,
    pub b: Ecs,
    pub c: AHashMap<Uuid, Player>,
    pub d: AHashMap<Uuid, Eid>,
    pub e: i64,
    pub f: i64,
    pub g: Difficulty,
    pub h: Dimension,
    pub i: u64,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Difficulty { Peaceful, Easy, Normal, Hard }
impl Difficulty {
    pub fn from_i32(n: i32) -> Self {
        match n { 0 => Self::Peaceful, 1 => Self::Easy, 3 => Self::Hard, _ => Self::Normal }
    }
    pub fn to_i32(self) -> i32 {
        match self { Self::Peaceful => 0, Self::Easy => 1, Self::Normal => 2, Self::Hard => 3 }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Dimension { Overworld, Nether, End }
impl Dimension {
    pub fn resource_key(self) -> &'static str {
        match self {
            Self::Overworld => "minecraft:overworld",
            Self::Nether    => "minecraft:the_nether",
            Self::End       => "minecraft:the_end",
        }
    }
}

impl Agsm {
    pub fn new() -> Self {
        Self {
            a: ChunkMap::new(),
            b: Ecs::new(),
            c: AHashMap::new(),
            d: AHashMap::new(),
            e: 0,
            f: 6000,
            g: Difficulty::Normal,
            h: Dimension::Overworld,
            i: 0,
        }
    }

    pub fn tick(&self) -> i64 { self.e }
    pub fn time_of_day(&self) -> i64 { self.f }
    pub fn difficulty(&self) -> Difficulty { self.g }
    pub fn dimension(&self) -> Dimension  { self.h }
    pub fn advance_tick(&mut self) {
        self.e += 1;
        self.f = (self.f + 1) % 24000;
        self.i += 1;
    }

    pub fn player(&self, uuid: Uuid) -> Option<&Player> { self.c.get(&uuid) }
    pub fn player_mut(&mut self, uuid: Uuid) -> Option<&mut Player> { self.c.get_mut(&uuid) }
    pub fn join_player(&mut self, mut p: Player) {
        let eid = p.eid();
        let uuid = p.uuid();
        let pos = p.pos().clone();
        self.b.insert(crate::entity::Entity::new(eid, Kind::Player, pos.x, pos.y, pos.z));
        self.d.insert(uuid, eid);
        self.c.insert(uuid, p);
    }

    pub fn leave_player(&mut self, uuid: Uuid) {
        if let Some(eid) = self.d.remove(&uuid) {
            self.b.remove(eid);
        }
        self.c.remove(&uuid);
    }

    pub fn player_count(&self) -> usize { self.c.len() }
    pub fn set_block(&mut self, pos: BlockPos, state: BlockState) -> crate::error::R<()> {
        self.a.set_block(pos, state)
    }

    pub fn get_block(&self, pos: BlockPos) -> BlockState {
        self.a.get_block(pos)
    }

    pub fn players(&self) -> impl Iterator<Item = &Player> { self.c.values() }
    pub fn players_mut(&mut self) -> impl Iterator<Item = &mut Player> { self.c.values_mut() }
}

impl Default for Agsm {
    fn default() -> Self { Self::new() }
}
