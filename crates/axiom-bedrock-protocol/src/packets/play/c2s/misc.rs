use bytes::Bytes;
use crate::codec::{Dec, Enc};
use crate::error::R;

pub const ID_TEXT:       u8 = 0x09;
pub const ID_ANIMATE:    u8 = 0x2C;
pub const ID_RESPAWN:    u8 = 0x35;
pub const ID_CHAT:       u8 = 0x09;
pub const ID_EMOTE:      u8 = 0x98;
pub const ID_PACKET_VIO: u8 = 0x7C;

#[derive(Debug, Clone)]
pub struct Text {
    pub a: u8,
    pub b: bool,
    pub c: String,
    pub d: String,
    pub e: Vec<String>,
    pub f: String,
    pub g: String,
    pub h: bool,
}

#[derive(Debug, Clone)]
pub struct Animate {
    pub a: i32,
    pub b: u64,
}

#[derive(Debug, Clone)]
pub struct Respawn {
    pub a: f32, pub b: f32, pub c: f32,
    pub d: u8,
    pub e: u64,
}

impl Text {
    pub fn encode(&self) -> Bytes {
        let mut cx = Enc::new();
        cx.u8(ID_TEXT);
        cx.u8(self.a);
        cx.bool(self.b);
        cx.str(&self.c);
        cx.str(&self.d);
        cx.varint(self.e.len() as u32);
        for p in &self.e { cx.str(p); }
        cx.str(&self.f);
        cx.str(&self.g);
        cx.bool(self.h);
        cx.finish()
    }

    pub fn decode(b: Bytes) -> R<Self> {
        let mut cx = Dec::new(b);
        cx.u8()?;
        let a = cx.u8()?;
        let bb = cx.bool()?;
        let c = cx.str()?;
        let d = cx.str()?;
        let n = cx.varint()? as usize;
        let mut e = Vec::with_capacity(n);
        for _ in 0..n { e.push(cx.str()?); }
        let f = cx.str()?;
        let g = cx.str()?;
        let h = cx.bool()?;
        Ok(Self { a, b: bb, c, d, e, f, g, h })
    }
}

impl Animate {
    pub fn encode(&self) -> Bytes {
        let mut cx = Enc::new();
        cx.u8(ID_ANIMATE);
        cx.varinti(self.a);
        cx.varint64(self.b);
        cx.finish()
    }

    pub fn decode(b: Bytes) -> R<Self> {
        let mut cx = Dec::new(b);
        cx.u8()?;
        Ok(Self { a: cx.varinti()?, b: cx.varint64()? })
    }
}

impl Respawn {
    pub fn encode(&self) -> Bytes {
        let mut cx = Enc::new();
        cx.u8(ID_RESPAWN);
        cx.f32(self.a); cx.f32(self.b); cx.f32(self.c);
        cx.u8(self.d);
        cx.varint64(self.e);
        cx.finish()
    }

    pub fn decode(b: Bytes) -> R<Self> {
        let mut cx = Dec::new(b);
        cx.u8()?;
        Ok(Self {
            a: cx.f32()?, b: cx.f32()?, c: cx.f32()?,
            d: cx.u8()?,
            e: cx.varint64()?,
        })
    }
}
