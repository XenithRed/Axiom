use bytes::Bytes;
use uuid::Uuid;
use crate::{
    entity::{Eid, Kind, MetaVal},
    net::pkt::Edition,
    player::GameMode,
    world::{BlockPos, BlockState, Biome},
};

#[derive(Debug, Clone)]
pub enum Event {
    PlayerJoin     { a: Uuid, b: String, c: Edition },
    PlayerLeave    { a: Uuid },
    PlayerMove     { a: Uuid, b: f64, c: f64, d: f64, e: f32, f: f32, g: bool },
    PlayerChat     { a: Uuid, b: String },
    PlayerCommand  { a: Uuid, b: String },
    PlayerGameMode { a: Uuid, b: GameMode },
    PlayerRespawn  { a: Uuid },
    PlayerHealth   { a: Uuid, b: f32, c: i32, d: f32 },
    EntitySpawn    { a: Eid, b: Kind, c: f64, d: f64, e: f64 },
    EntityDespawn  { a: Eid },
    EntityMove     { a: Eid, b: f64, c: f64, d: f64, e: f32, f: f32 },
    EntityMeta     { a: Eid, b: u8, c: MetaVal },
    EntityVelocity { a: Eid, b: f64, c: f64, d: f64 },
    BlockChange    { a: BlockPos, b: BlockState },
    ChunkLoad      { a: i32, b: i32, c: Bytes },
    ChunkUnload    { a: i32, b: i32 },
    TimeChange     { a: i64 },
    DifficultyChange { a: i32 },
    SystemMsg      { a: String, b: bool },
}

impl Event {
    pub fn player_join(uuid: Uuid, name: String, edition: Edition) -> Self {
        Self::PlayerJoin { a: uuid, b: name, c: edition }
    }

    pub fn player_move(uuid: Uuid, x: f64, y: f64, z: f64, yaw: f32, pitch: f32, on_ground: bool) -> Self {
        Self::PlayerMove { a: uuid, b: x, c: y, d: z, e: yaw, f: pitch, g: on_ground }
    }

    pub fn block_change(pos: BlockPos, state: BlockState) -> Self {
        Self::BlockChange { a: pos, b: state }
    }
}
