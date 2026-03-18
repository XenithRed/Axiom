use bytes::Bytes;
use crate::codec::{Dec, Enc};
use crate::error::R;

pub const ID_PLAYER_LIST:   u8 = 0x3F;
pub const ID_SET_SPAWN:     u8 = 0x43;
pub const ID_GAME_MODE:     u8 = 0x31;
pub const ID_TRANSFER:      u8 = 0x55;
pub const ID_DISCONNECT:    u8 = 0x05;
pub const ID_PLAY_STATUS:   u8 = 0x02;
pub const ID_NETWORK_SETTINGS: u8 = 0x8F;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum PlayStatus {
    LoginSuccess    = 0,
    FailedClient    = 1,
    FailedSpawn     = 2,
    PlayerSpawn     = 3,
    LoginFailedInvalidTenant = 4,
    LoginFailedVanillaEdu    = 5,
    LoginFailedEduVanilla    = 6,
    LoginFailedServerFull    = 7,
}

impl PlayStatus {
    pub fn from_u32(n: u32) -> Option<Self> {
        match n {
            0 => Some(Self::LoginSuccess),
            1 => Some(Self::FailedClient),
            2 => Some(Self::FailedSpawn),
            3 => Some(Self::PlayerSpawn),
            4 => Some(Self::LoginFailedInvalidTenant),
            5 => Some(Self::LoginFailedVanillaEdu),
            6 => Some(Self::LoginFailedEduVanilla),
            7 => Some(Self::LoginFailedServerFull),
            _ => None,
        }
    }
}

#[derive(Debug, Clone)]
pub struct PlayStatusPkt {
    pub a: PlayStatus,
}

#[derive(Debug, Clone)]
pub struct Disconnect {
    pub a: bool,
    pub b: String,
}

#[derive(Debug, Clone)]
pub struct SetSpawn {
    pub a: i32, pub b: i32, pub c: i32,
    pub d: i32,
    pub e: String,
}

#[derive(Debug, Clone)]
pub struct GameMode {
    pub a: i32,
}

#[derive(Debug, Clone)]
pub struct Transfer {
    pub a: String,
    pub b: u16,
    pub c: bool,
}

#[derive(Debug, Clone)]
pub struct NetworkSettings {
    pub a: u16,
    pub b: u16,
    pub c: bool,
    pub d: u32,
    pub e: u32,
}

impl PlayStatusPkt {
    pub fn encode(&self) -> Bytes {
        let mut cx = Enc::new();
        cx.u8(ID_PLAY_STATUS);
        cx.u32(self.a as u32);
        cx.finish()
    }

    pub fn decode(b: Bytes) -> R<Self> {
        let mut cx = Dec::new(b);
        cx.u8()?;
        let n = cx.u32()?;
        Ok(Self { a: PlayStatus::from_u32(n).unwrap_or(PlayStatus::LoginSuccess) })
    }
}

impl Disconnect {
    pub fn encode(&self) -> Bytes {
        let mut cx = Enc::new();
        cx.u8(ID_DISCONNECT);
        cx.bool(self.a);
        if !self.a { cx.str(&self.b); }
        cx.finish()
    }

    pub fn decode(b: Bytes) -> R<Self> {
        let mut cx = Dec::new(b);
        cx.u8()?;
        let a = cx.bool()?;
        let bb = if !a { cx.str()? } else { String::new() };
        Ok(Self { a, b: bb })
    }
}

impl SetSpawn {
    pub fn encode(&self) -> Bytes {
        let mut cx = Enc::new();
        cx.u8(ID_SET_SPAWN);
        cx.varinti(self.a); cx.varinti(self.b); cx.varinti(self.c);
        cx.varinti(self.d);
        cx.str(&self.e);
        cx.finish()
    }

    pub fn decode(b: Bytes) -> R<Self> {
        let mut cx = Dec::new(b);
        cx.u8()?;
        Ok(Self {
            a: cx.varinti()?, b: cx.varinti()?, c: cx.varinti()?,
            d: cx.varinti()?,
            e: cx.str()?,
        })
    }
}

impl GameMode {
    pub fn encode(&self) -> Bytes {
        let mut cx = Enc::new();
        cx.u8(ID_GAME_MODE);
        cx.varinti(self.a);
        cx.finish()
    }

    pub fn decode(b: Bytes) -> R<Self> {
        let mut cx = Dec::new(b);
        cx.u8()?;
        Ok(Self { a: cx.varinti()? })
    }
}

impl Transfer {
    pub fn encode(&self) -> Bytes {
        let mut cx = Enc::new();
        cx.u8(ID_TRANSFER);
        cx.str(&self.a);
        cx.u16(self.b);
        cx.bool(self.c);
        cx.finish()
    }

    pub fn decode(b: Bytes) -> R<Self> {
        let mut cx = Dec::new(b);
        cx.u8()?;
        Ok(Self { a: cx.str()?, b: cx.u16()?, c: cx.bool()? })
    }
}

impl NetworkSettings {
    pub fn encode(&self) -> Bytes {
        let mut cx = Enc::new();
        cx.u8(ID_NETWORK_SETTINGS);
        cx.u16(self.a);
        cx.u16(self.b);
        cx.bool(self.c);
        cx.u32(self.d);
        cx.u32(self.e);
        cx.finish()
    }

    pub fn decode(b: Bytes) -> R<Self> {
        let mut cx = Dec::new(b);
        cx.u8()?;
        Ok(Self {
            a: cx.u16()?,
            b: cx.u16()?,
            c: cx.bool()?,
            d: cx.u32()?,
            e: cx.u32()?,
        })
    }
}
