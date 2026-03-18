use bytes::Bytes;
use crate::codec::{Dec, Enc};
use crate::nbt::Tag;
use crate::error::R;

pub const ID_LOGIN:            i32 = 0x2B;
pub const ID_RESPAWN:          i32 = 0x47;
pub const ID_PLAYER_POSITION:  i32 = 0x40;
pub const ID_PLAYER_ABILITIES: i32 = 0x38;
pub const ID_SET_HEALTH:       i32 = 0x59;
pub const ID_SET_EXPERIENCE:   i32 = 0x58;
pub const ID_GAME_EVENT:       i32 = 0x22;
pub const ID_PLAYER_LIST_ADD:  i32 = 0x3E;
pub const ID_DISCONNECT:       i32 = 0x1C;

#[derive(Debug, Clone)]
pub struct Login {
    pub a: i32,
    pub b: bool,
    pub c: Vec<String>,
    pub d: i32,
    pub e: i32,
    pub f: i32,
    pub g: u8,
    pub h: String,
    pub i: bool,
    pub j: bool,
    pub k: bool,
    pub l: i32,
    pub m: i32,
    pub n: bool,
    pub o: bool,
    pub p: bool,
    pub q: i32,
}

#[derive(Debug, Clone)]
pub struct Respawn {
    pub a: String,
    pub b: String,
    pub c: i64,
    pub d: u8,
    pub e: u8,
    pub f: bool,
    pub g: bool,
    pub h: u8,
    pub i: Option<()>,
    pub j: i32,
    pub k: i32,
}

#[derive(Debug, Clone)]
pub struct PlayerPosition {
    pub a: f64, pub b: f64, pub c: f64,
    pub d: f32, pub e: f32,
    pub f: u8,
    pub g: i32,
}

#[derive(Debug, Clone)]
pub struct PlayerAbilities {
    pub a: u8,
    pub b: f32,
    pub c: f32,
}

#[derive(Debug, Clone)]
pub struct SetHealth { pub a: f32, pub b: i32, pub c: f32 }

#[derive(Debug, Clone)]
pub struct SetExperience { pub a: f32, pub b: i32, pub c: i32 }

#[derive(Debug, Clone)]
pub struct GameEvent { pub a: u8, pub b: f32 }

#[derive(Debug, Clone)]
pub struct Disconnect { pub a: String }

impl Login {
    pub fn encode(&self) -> Bytes {
        let mut cx = Enc::new(); cx.var32(ID_LOGIN);
        cx.i32(self.a); cx.bool(self.b);
        cx.var32(self.c.len() as i32);
        for d in &self.c { cx.str(d); }
        cx.var32(self.d); cx.var32(self.e); cx.var32(self.f);
        cx.u8(self.g); cx.str(&self.h);
        cx.bool(self.i); cx.bool(self.j); cx.bool(self.k);
        cx.var32(self.l); cx.var32(self.m);
        cx.bool(self.n); cx.bool(self.o); cx.bool(self.p);
        cx.var32(self.q);
        cx.finish()
    }
    pub fn decode(b: Bytes) -> R<Self> {
        let mut cx = Dec::new(b); cx.var32()?;
        let a = cx.i32()?; let bb = cx.bool()?;
        let dn = cx.var32()? as usize;
        let mut c = Vec::with_capacity(dn);
        for _ in 0..dn { c.push(cx.str()?); }
        Ok(Self {
            a, b: bb, c,
            d: cx.var32()?, e: cx.var32()?, f: cx.var32()?,
            g: cx.u8()?, h: cx.str()?,
            i: cx.bool()?, j: cx.bool()?, k: cx.bool()?,
            l: cx.var32()?, m: cx.var32()?,
            n: cx.bool()?, o: cx.bool()?, p: cx.bool()?,
            q: cx.var32()?,
        })
    }
}

impl PlayerPosition {
    pub fn encode(&self) -> Bytes {
        let mut cx = Enc::new(); cx.var32(ID_PLAYER_POSITION);
        cx.f64(self.a); cx.f64(self.b); cx.f64(self.c);
        cx.f32(self.d); cx.f32(self.e);
        cx.u8(self.f); cx.var32(self.g);
        cx.finish()
    }
    pub fn decode(b: Bytes) -> R<Self> {
        let mut cx = Dec::new(b); cx.var32()?;
        Ok(Self {
            a: cx.f64()?, b: cx.f64()?, c: cx.f64()?,
            d: cx.f32()?, e: cx.f32()?,
            f: cx.u8()?, g: cx.var32()?,
        })
    }
}

impl PlayerAbilities {
    pub fn encode(&self) -> Bytes {
        let mut cx = Enc::new(); cx.var32(ID_PLAYER_ABILITIES);
        cx.u8(self.a); cx.f32(self.b); cx.f32(self.c); cx.finish()
    }
    pub fn decode(b: Bytes) -> R<Self> {
        let mut cx = Dec::new(b); cx.var32()?;
        Ok(Self { a: cx.u8()?, b: cx.f32()?, c: cx.f32()? })
    }
}

impl SetHealth {
    pub fn encode(&self) -> Bytes {
        let mut cx = Enc::new(); cx.var32(ID_SET_HEALTH);
        cx.f32(self.a); cx.var32(self.b); cx.f32(self.c); cx.finish()
    }
    pub fn decode(b: Bytes) -> R<Self> {
        let mut cx = Dec::new(b); cx.var32()?;
        Ok(Self { a: cx.f32()?, b: cx.var32()?, c: cx.f32()? })
    }
}

impl SetExperience {
    pub fn encode(&self) -> Bytes {
        let mut cx = Enc::new(); cx.var32(ID_SET_EXPERIENCE);
        cx.f32(self.a); cx.var32(self.b); cx.var32(self.c); cx.finish()
    }
    pub fn decode(b: Bytes) -> R<Self> {
        let mut cx = Dec::new(b); cx.var32()?;
        Ok(Self { a: cx.f32()?, b: cx.var32()?, c: cx.var32()? })
    }
}

impl GameEvent {
    pub fn encode(&self) -> Bytes {
        let mut cx = Enc::new(); cx.var32(ID_GAME_EVENT);
        cx.u8(self.a); cx.f32(self.b); cx.finish()
    }
    pub fn decode(b: Bytes) -> R<Self> {
        let mut cx = Dec::new(b); cx.var32()?;
        Ok(Self { a: cx.u8()?, b: cx.f32()? })
    }
}

impl Disconnect {
    pub fn encode(&self) -> Bytes {
        let mut cx = Enc::new(); cx.var32(ID_DISCONNECT);
        cx.str(&self.a); cx.finish()
    }
    pub fn decode(b: Bytes) -> R<Self> {
        let mut cx = Dec::new(b); cx.var32()?;
        Ok(Self { a: cx.str()? })
    }
}
