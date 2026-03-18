use bytes::Bytes;
use crate::codec::{Dec, Enc};
use crate::error::R;

pub const ID_ADD_PLAYER:   u8 = 0x0C;
pub const ID_ADD_ENTITY:   u8 = 0x0D;
pub const ID_REMOVE_ENTITY: u8 = 0x0E;
pub const ID_MOVE_ENTITY:  u8 = 0x11;
pub const ID_SET_ENTITY_DATA: u8 = 0x27;
pub const ID_SET_HEALTH:   u8 = 0x26;

#[derive(Debug, Clone)]
pub struct AddPlayer {
    pub a: [u8; 16],
    pub b: String,
    pub c: u64,
    pub d: i64,
    pub e: f32, pub f: f32, pub g: f32,
    pub h: f32, pub i: f32, pub j: f32,
}

#[derive(Debug, Clone)]
pub struct RemoveEntity {
    pub a: i64,
}

#[derive(Debug, Clone)]
pub struct MoveEntity {
    pub a: u64,
    pub b: f32, pub c: f32, pub d: f32,
    pub e: f32, pub f: f32, pub g: f32,
    pub h: bool,
    pub i: bool,
}

#[derive(Debug, Clone)]
pub struct SetHealth {
    pub a: i32,
}

impl AddPlayer {
    pub fn encode(&self) -> Bytes {
        let mut cx = Enc::new();
        cx.u8(ID_ADD_PLAYER);
        cx.bytes(&self.a);
        cx.str(&self.b);
        cx.varint64(self.c);
        cx.varinti64(self.d);
        cx.f32(self.e); cx.f32(self.f); cx.f32(self.g);
        cx.f32(self.h); cx.f32(self.i); cx.f32(self.j);
        cx.finish()
    }

    pub fn decode(b: Bytes) -> R<Self> {
        let mut cx = Dec::new(b);
        cx.u8()?;
        let uid_bytes = cx.slice(16)?;
        let mut a = [0u8; 16];
        a.copy_from_slice(&uid_bytes);
        Ok(Self {
            a,
            b: cx.str()?,
            c: cx.varint64()?,
            d: cx.varinti64()?,
            e: cx.f32()?, f: cx.f32()?, g: cx.f32()?,
            h: cx.f32()?, i: cx.f32()?, j: cx.f32()?,
        })
    }
}

impl RemoveEntity {
    pub fn encode(&self) -> Bytes {
        let mut cx = Enc::new();
        cx.u8(ID_REMOVE_ENTITY);
        cx.varinti64(self.a);
        cx.finish()
    }

    pub fn decode(b: Bytes) -> R<Self> {
        let mut cx = Dec::new(b);
        cx.u8()?;
        Ok(Self { a: cx.varinti64()? })
    }
}

impl MoveEntity {
    pub fn encode(&self) -> Bytes {
        let mut cx = Enc::new();
        cx.u8(ID_MOVE_ENTITY);
        cx.varint64(self.a);
        cx.f32(self.b); cx.f32(self.c); cx.f32(self.d);
        cx.f32(self.e); cx.f32(self.f); cx.f32(self.g);
        cx.bool(self.h);
        cx.bool(self.i);
        cx.finish()
    }

    pub fn decode(b: Bytes) -> R<Self> {
        let mut cx = Dec::new(b);
        cx.u8()?;
        Ok(Self {
            a: cx.varint64()?,
            b: cx.f32()?, c: cx.f32()?, d: cx.f32()?,
            e: cx.f32()?, f: cx.f32()?, g: cx.f32()?,
            h: cx.bool()?,
            i: cx.bool()?,
        })
    }
}

impl SetHealth {
    pub fn encode(&self) -> Bytes {
        let mut cx = Enc::new();
        cx.u8(ID_SET_HEALTH);
        cx.varinti(self.a);
        cx.finish()
    }

    pub fn decode(b: Bytes) -> R<Self> {
        let mut cx = Dec::new(b);
        cx.u8()?;
        Ok(Self { a: cx.varinti()? })
    }
}
