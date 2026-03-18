use bytes::Bytes;
use crate::codec::{Dec, Enc};
use crate::error::R;

pub const ID_SPAWN_ENTITY:     i32 = 0x01;
pub const ID_REMOVE_ENTITIES:  i32 = 0x42;
pub const ID_MOVE_ENTITY_POS:  i32 = 0x2C;
pub const ID_MOVE_ENTITY_ROT:  i32 = 0x2E;
pub const ID_MOVE_ENTITY_FULL: i32 = 0x2D;
pub const ID_TELEPORT_ENTITY:  i32 = 0x6C;
pub const ID_SET_ENTITY_META:  i32 = 0x58;
pub const ID_SET_ENTITY_VEL:   i32 = 0x5C;
pub const ID_ENTITY_EFFECT:    i32 = 0x76;
pub const ID_REMOVE_EFFECT:    i32 = 0x43;

#[derive(Debug, Clone)]
pub struct SpawnEntity {
    pub a: i32,
    pub b: u128,
    pub c: i32,
    pub d: f64, pub e: f64, pub f: f64,
    pub g: u8,  pub h: u8,
    pub i: i32,
    pub j: i16, pub k: i16, pub l: i16,
}

#[derive(Debug, Clone)]
pub struct RemoveEntities { pub a: Vec<i32> }

#[derive(Debug, Clone)]
pub struct MoveEntityPos {
    pub a: i32,
    pub b: i16, pub c: i16, pub d: i16,
    pub e: bool,
}

#[derive(Debug, Clone)]
pub struct MoveEntityRot { pub a: i32, pub b: u8, pub c: u8, pub d: bool }

#[derive(Debug, Clone)]
pub struct MoveEntityFull {
    pub a: i32,
    pub b: i16, pub c: i16, pub d: i16,
    pub e: u8,  pub f: u8,
    pub g: bool,
}

#[derive(Debug, Clone)]
pub struct TeleportEntity {
    pub a: i32,
    pub b: f64, pub c: f64, pub d: f64,
    pub e: f32, pub f: f32,
    pub g: i16, pub h: i16, pub i: i16,
    pub j: bool,
}

#[derive(Debug, Clone)]
pub struct SetEntityVelocity { pub a: i32, pub b: i16, pub c: i16, pub d: i16 }

impl SpawnEntity {
    pub fn encode(&self) -> Bytes {
        let mut cx = Enc::new(); cx.var32(ID_SPAWN_ENTITY);
        cx.var32(self.a); cx.uuid(self.b); cx.var32(self.c);
        cx.f64(self.d); cx.f64(self.e); cx.f64(self.f);
        cx.angle(self.g as f32 / 256.0 * 360.0);
        cx.angle(self.h as f32 / 256.0 * 360.0);
        cx.var32(self.i);
        cx.i16(self.j); cx.i16(self.k); cx.i16(self.l);
        cx.finish()
    }
    pub fn decode(b: Bytes) -> R<Self> {
        let mut cx = Dec::new(b); cx.var32()?;
        Ok(Self {
            a: cx.var32()?, b: cx.uuid()?, c: cx.var32()?,
            d: cx.f64()?, e: cx.f64()?, f: cx.f64()?,
            g: cx.u8()?, h: cx.u8()?,
            i: cx.var32()?,
            j: cx.i16()?, k: cx.i16()?, l: cx.i16()?,
        })
    }
}

impl RemoveEntities {
    pub fn encode(&self) -> Bytes {
        let mut cx = Enc::new(); cx.var32(ID_REMOVE_ENTITIES);
        cx.var32(self.a.len() as i32);
        for &id in &self.a { cx.var32(id); }
        cx.finish()
    }
    pub fn decode(b: Bytes) -> R<Self> {
        let mut cx = Dec::new(b); cx.var32()?;
        let n = cx.var32()? as usize;
        let mut v = Vec::with_capacity(n);
        for _ in 0..n { v.push(cx.var32()?); }
        Ok(Self { a: v })
    }
}

impl MoveEntityPos {
    pub fn encode(&self) -> Bytes {
        let mut cx = Enc::new(); cx.var32(ID_MOVE_ENTITY_POS);
        cx.var32(self.a);
        cx.i16(self.b); cx.i16(self.c); cx.i16(self.d);
        cx.bool(self.e); cx.finish()
    }
    pub fn decode(b: Bytes) -> R<Self> {
        let mut cx = Dec::new(b); cx.var32()?;
        Ok(Self { a: cx.var32()?, b: cx.i16()?, c: cx.i16()?, d: cx.i16()?, e: cx.bool()? })
    }
}

impl MoveEntityRot {
    pub fn encode(&self) -> Bytes {
        let mut cx = Enc::new(); cx.var32(ID_MOVE_ENTITY_ROT);
        cx.var32(self.a); cx.u8(self.b); cx.u8(self.c); cx.bool(self.d); cx.finish()
    }
    pub fn decode(b: Bytes) -> R<Self> {
        let mut cx = Dec::new(b); cx.var32()?;
        Ok(Self { a: cx.var32()?, b: cx.u8()?, c: cx.u8()?, d: cx.bool()? })
    }
}

impl SetEntityVelocity {
    pub fn encode(&self) -> Bytes {
        let mut cx = Enc::new(); cx.var32(ID_SET_ENTITY_VEL);
        cx.var32(self.a); cx.i16(self.b); cx.i16(self.c); cx.i16(self.d); cx.finish()
    }
    pub fn decode(b: Bytes) -> R<Self> {
        let mut cx = Dec::new(b); cx.var32()?;
        Ok(Self { a: cx.var32()?, b: cx.i16()?, c: cx.i16()?, d: cx.i16()? })
    }
}
