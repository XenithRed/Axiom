use bytes::Bytes;
use crate::codec::{Dec, Enc};
use crate::error::R;

pub const ID_LEVEL_CHUNK:    u8 = 0x3A;
pub const ID_UPDATE_BLOCK:   u8 = 0x15;
pub const ID_CHUNK_RADIUS:   u8 = 0x45;
pub const ID_NETWORK_CHUNK:  u8 = 0xA8;

#[derive(Debug, Clone)]
pub struct LevelChunk {
    pub a: i32,
    pub b: i32,
    pub c: u32,
    pub d: Option<u32>,
    pub e: bool,
    pub f: Bytes,
}

#[derive(Debug, Clone)]
pub struct UpdateBlock {
    pub a: i32, pub b: i32, pub c: i32,
    pub d: u32,
    pub e: u32,
}

#[derive(Debug, Clone)]
pub struct ChunkRadiusUpdated {
    pub a: i32,
}

impl LevelChunk {
    pub fn encode(&self) -> Bytes {
        let mut cx = Enc::new();
        cx.u8(ID_LEVEL_CHUNK);
        cx.varinti(self.a);
        cx.varinti(self.b);
        cx.varint(self.c);
        if let Some(n) = self.d {
            cx.bool(true);
            cx.varint(n);
        } else {
            cx.bool(false);
        }
        cx.bool(self.e);
        cx.varint(self.f.len() as u32);
        cx.bytes(&self.f);
        cx.finish()
    }

    pub fn decode(b: Bytes) -> R<Self> {
        let mut cx = Dec::new(b);
        cx.u8()?;
        let a = cx.varinti()?;
        let bb = cx.varinti()?;
        let c = cx.varint()?;
        let has_sub = cx.bool()?;
        let d = if has_sub { Some(cx.varint()?) } else { None };
        let e = cx.bool()?;
        let n = cx.varint()? as usize;
        let f = cx.slice(n)?;
        Ok(Self { a, b: bb, c, d, e, f })
    }
}

impl UpdateBlock {
    pub fn encode(&self) -> Bytes {
        let mut cx = Enc::new();
        cx.u8(ID_UPDATE_BLOCK);
        cx.varinti(self.a); cx.varinti(self.b); cx.varinti(self.c);
        cx.varint(self.d);
        cx.varint(self.e);
        cx.finish()
    }

    pub fn decode(b: Bytes) -> R<Self> {
        let mut cx = Dec::new(b);
        cx.u8()?;
        Ok(Self {
            a: cx.varinti()?, b: cx.varinti()?, c: cx.varinti()?,
            d: cx.varint()?,
            e: cx.varint()?,
        })
    }
}

impl ChunkRadiusUpdated {
    pub fn encode(&self) -> Bytes {
        let mut cx = Enc::new();
        cx.u8(ID_CHUNK_RADIUS);
        cx.varinti(self.a);
        cx.finish()
    }

    pub fn decode(b: Bytes) -> R<Self> {
        let mut cx = Dec::new(b);
        cx.u8()?;
        Ok(Self { a: cx.varinti()? })
    }
}
