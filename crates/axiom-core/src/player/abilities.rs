use bitflags::bitflags;
use serde::{Deserialize, Serialize};

bitflags! {
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
    pub struct Flags: u8 {
        const INVULNERABLE = 0x01;
        const FLYING       = 0x02;
        const ALLOW_FLYING = 0x04;
        const INSTANT_BUILD = 0x08;
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct Abilities {
    pub a: Flags,
    pub b: f32,
    pub c: f32,
}

impl Abilities {
    pub fn survival() -> Self {
        Self { a: Flags::empty(), b: 0.05, c: 0.1 }
    }

    pub fn creative() -> Self {
        Self {
            a: Flags::INVULNERABLE | Flags::FLYING | Flags::ALLOW_FLYING | Flags::INSTANT_BUILD,
            b: 0.05,
            c: 0.1,
        }
    }

    pub fn spectator() -> Self {
        Self {
            a: Flags::INVULNERABLE | Flags::FLYING | Flags::ALLOW_FLYING,
            b: 0.05,
            c: 0.1,
        }
    }

    pub fn is_flying(self) -> bool     { self.a.contains(Flags::FLYING) }
    pub fn can_fly(self) -> bool       { self.a.contains(Flags::ALLOW_FLYING) }
    pub fn invulnerable(self) -> bool  { self.a.contains(Flags::INVULNERABLE) }
    pub fn instant_build(self) -> bool { self.a.contains(Flags::INSTANT_BUILD) }

    pub fn set_flying(&mut self, v: bool) { self.a.set(Flags::FLYING, v); }
}

impl Default for Abilities {
    fn default() -> Self { Self::survival() }
}
