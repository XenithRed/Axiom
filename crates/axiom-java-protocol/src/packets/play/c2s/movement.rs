use bytes::Bytes;
use crate::codec::{Dec, Enc};
use crate::error::R;

pub const ID_CONFIRM_TP:        i32 = 0x00;
pub const ID_MOVE_VEHICLE:      i32 = 0x17;
pub const ID_MOVE_PLAYER_POS:   i32 = 0x1A;
pub const ID_MOVE_PLAYER_ROT:   i32 = 0x1B;
pub const ID_MOVE_PLAYER_FULL:  i32 = 0x1C;
pub const ID_MOVE_PLAYER_ON:    i32 = 0x1D;

#[derive(Debug, Clone)]
pub struct ConfirmTp { pub a: i32 }

#[derive(Debug, Clone)]
pub struct MovePlayerPos {
    pub a: f64, pub b: f64, pub c: f64,
    pub d: u8,
    pub e: bool,
}

#[derive(Debug, Clone)]
pub struct MovePlayerRot {
    pub a: f32, pub b: f32,
    pub c: u8,
    pub d: bool,
}

#[derive(Debug, Clone)]
pub struct MovePlayerFull {
    pub a: f64, pub b: f64, pub c: f64,
    pub d: f32, pub e: f32,
    pub f: u8,
    pub g: bool,
    pub h: bool,
}

#[derive(Debug, Clone)]
pub struct MovePlayerOnGround { pub a: bool, pub b: u8 }

impl ConfirmTp {
    pub fn encode(&self) -> Bytes {
        let mut cx = Enc::new(); cx.var32(ID_CONFIRM_TP); cx.var32(self.a); cx.finish()
    }
    pub fn decode(b: Bytes) -> R<Self> {
        let mut cx = Dec::new(b); cx.var32()?; Ok(Self { a: cx.var32()? })
    }
}

impl MovePlayerPos {
    pub fn encode(&self) -> Bytes {
        let mut cx = Enc::new(); cx.var32(ID_MOVE_PLAYER_POS);
        cx.f64(self.a); cx.f64(self.b); cx.f64(self.c);
        cx.u8(self.d); cx.bool(self.e); cx.finish()
    }
    pub fn decode(b: Bytes) -> R<Self> {
        let mut cx = Dec::new(b); cx.var32()?;
        Ok(Self { a: cx.f64()?, b: cx.f64()?, c: cx.f64()?, d: cx.u8()?, e: cx.bool()? })
    }
}

impl MovePlayerRot {
    pub fn encode(&self) -> Bytes {
        let mut cx = Enc::new(); cx.var32(ID_MOVE_PLAYER_ROT);
        cx.f32(self.a); cx.f32(self.b); cx.u8(self.c); cx.bool(self.d); cx.finish()
    }
    pub fn decode(b: Bytes) -> R<Self> {
        let mut cx = Dec::new(b); cx.var32()?;
        Ok(Self { a: cx.f32()?, b: cx.f32()?, c: cx.u8()?, d: cx.bool()? })
    }
}

impl MovePlayerFull {
    pub fn encode(&self) -> Bytes {
        let mut cx = Enc::new(); cx.var32(ID_MOVE_PLAYER_FULL);
        cx.f64(self.a); cx.f64(self.b); cx.f64(self.c);
        cx.f32(self.d); cx.f32(self.e);
        cx.u8(self.f); cx.bool(self.g); cx.bool(self.h); cx.finish()
    }
    pub fn decode(b: Bytes) -> R<Self> {
        let mut cx = Dec::new(b); cx.var32()?;
        Ok(Self {
            a: cx.f64()?, b: cx.f64()?, c: cx.f64()?,
            d: cx.f32()?, e: cx.f32()?,
            f: cx.u8()?, g: cx.bool()?, h: cx.bool()?,
        })
    }
}

impl MovePlayerOnGround {
    pub fn encode(&self) -> Bytes {
        let mut cx = Enc::new(); cx.var32(ID_MOVE_PLAYER_ON);
        cx.bool(self.a); cx.u8(self.b); cx.finish()
    }
    pub fn decode(b: Bytes) -> R<Self> {
        let mut cx = Dec::new(b); cx.var32()?;
        Ok(Self { a: cx.bool()?, b: cx.u8()? })
    }
}
