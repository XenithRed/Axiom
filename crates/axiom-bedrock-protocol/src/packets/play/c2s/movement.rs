use bytes::Bytes;
use crate::codec::{Dec, Enc};
use crate::error::R;

pub const ID_MOVE_PLAYER: u8 = 0x13;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum MoveMode { Normal = 0, Reset = 1, Teleport = 2, Rotation = 3 }

impl MoveMode {
    pub fn from_u8(n: u8) -> Self {
        match n { 1 => Self::Reset, 2 => Self::Teleport, 3 => Self::Rotation, _ => Self::Normal }
    }
}

#[derive(Debug, Clone)]
pub struct MovePlayer {
    pub a: u64,
    pub b: f32, pub c: f32, pub d: f32,
    pub e: f32, pub f: f32, pub g: f32,
    pub h: MoveMode,
    pub i: bool,
    pub j: bool,
    pub k: u64,
    pub l: u64,
}

impl MovePlayer {
    pub fn encode(&self) -> Bytes {
        let mut cx = Enc::new();
        cx.u8(ID_MOVE_PLAYER);
        cx.varint64(self.a);
        cx.f32(self.b); cx.f32(self.c); cx.f32(self.d);
        cx.f32(self.e); cx.f32(self.f); cx.f32(self.g);
        cx.u8(self.h as u8);
        cx.bool(self.i);
        cx.bool(self.j);
        cx.u64(self.k);
        cx.u64(self.l);
        cx.finish()
    }

    pub fn decode(b: Bytes) -> R<Self> {
        let mut cx = Dec::new(b);
        cx.u8()?;
        Ok(Self {
            a: cx.varint64()?,
            b: cx.f32()?, c: cx.f32()?, d: cx.f32()?,
            e: cx.f32()?, f: cx.f32()?, g: cx.f32()?,
            h: MoveMode::from_u8(cx.u8()?),
            i: cx.bool()?,
            j: cx.bool()?,
            k: cx.u64()?,
            l: cx.u64()?,
        })
    }
}
