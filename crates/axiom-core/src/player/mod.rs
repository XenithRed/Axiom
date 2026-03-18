pub mod abilities;
pub mod inv;
pub mod pos;
pub mod state;

pub use abilities::{Abilities, Flags as AbilityFlags};
pub use inv::{Inventory, ItemStack, HOTBAR_START, INV_SIZE, OFFHAND};
pub use pos::Pos;
pub use state::{ConnState, GameMode, Stats};

use serde::{Deserialize, Serialize};
use uuid::Uuid;
use crate::entity::id::Eid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Player {
    pub a: Eid,
    pub b: Uuid,
    pub c: String,
    pub d: Pos,
    pub e: Abilities,
    pub f: Inventory,
    pub g: Stats,
    pub h: GameMode,
    pub i: ConnState,
    pub j: String,
    pub k: u16,
}

impl Player {
    pub fn new(uuid: Uuid, name: String, addr: String, port: u16) -> Self {
        Self {
            a: Eid::next(),
            b: uuid,
            c: name,
            d: Pos::default(),
            e: Abilities::survival(),
            f: Inventory::new(),
            g: Stats::default(),
            h: GameMode::Survival,
            i: ConnState::Login,
            j: addr,
            k: port,
        }
    }

    pub fn eid(&self)  -> Eid      { self.a }
    pub fn uuid(&self) -> Uuid     { self.b }
    pub fn name(&self) -> &str     { &self.c }
    pub fn pos(&self)  -> &Pos     { &self.d }
    pub fn mode(&self) -> GameMode { self.h }
    pub fn state(&self) -> ConnState { self.i }
}
