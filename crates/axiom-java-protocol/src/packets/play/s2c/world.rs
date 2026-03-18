use bytes::Bytes;
use crate::codec::{Dec, Enc};
use crate::error::R;

pub const ID_CHUNK_DATA:       i32 = 0x27;
pub const ID_BLOCK_UPDATE:     i32 = 0x09;
pub const ID_MULTI_BLOCK:      i32 = 0x0B;
pub const ID_SET_CENTER_CHUNK: i32 = 0x54;
pub const ID_CHUNK_BATCH_START: i32 = 0x0C;
pub const ID_CHUNK_BATCH_END:  i32 = 0x0D;
pub const ID_UNLOAD_CHUNK:     i32 = 0x1F;
pub const ID_SET_TIME:         i32 = 0x5A;
pub const ID_LEVEL_EVENT:      i32 = 0x2B;
pub const ID_PARTICLE:         i32 = 0x29;

#[derive(Debug, Clone)]
pub struct ChunkData {
    pub a: i32,
    pub b: i32,
    pub c: Bytes,
    pub d: Bytes,
}

#[derive(Debug, Clone)]
pub struct BlockUpdate {
    pub a: i32, pub b: i16, pub c: i32,
    pub d: i32,
}

#[derive(Debug, Clone)]
pub struct SetCenterChunk { pub a: i32, pub b: i32 }

#[derive(Debug, Clone)]
pub struct UnloadChunk { pub a: i32, pub b: i32 }

#[derive(Debug, Clone)]
pub struct SetTime { pub a: i64, pub b: i64, pub c: bool }

#[derive(Debug, Clone)]
pub struct LevelEvent { pub a: i32, pub b: i32, pub c: i16, pub d: i32, pub e: i32, pub f: bool }

impl ChunkData {
    pub fn encode(&self) -> Bytes {
        let mut cx = Enc::new(); cx.var32(ID_CHUNK_DATA);
        cx.i32(self.a); cx.i32(self.b);
        cx.bytes_prefixed(&self.c);
        cx.bytes_prefixed(&self.d);
        cx.finish()
    }
    pub fn decode(b: Bytes) -> R<Self> {
        let mut cx = Dec::new(b); cx.var32()?;
        let a = cx.i32()?; let bb = cx.i32()?;
        let c = cx.bytes_prefixed()?;
        let d = cx.bytes_prefixed()?;
        Ok(Self { a, b: bb, c, d })
    }
}

impl BlockUpdate {
    pub fn encode(&self) -> Bytes {
        let mut cx = Enc::new(); cx.var32(ID_BLOCK_UPDATE);
        cx.pos(self.a, self.b, self.c);
        cx.var32(self.d);
        cx.finish()
    }
    pub fn decode(b: Bytes) -> R<Self> {
        let mut cx = Dec::new(b); cx.var32()?;
        let (a, bb, c) = cx.pos()?;
        let d = cx.var32()?;
        Ok(Self { a, b: bb, c, d })
    }
}

impl SetCenterChunk {
    pub fn encode(&self) -> Bytes {
        let mut cx = Enc::new(); cx.var32(ID_SET_CENTER_CHUNK);
        cx.var32(self.a); cx.var32(self.b); cx.finish()
    }
    pub fn decode(b: Bytes) -> R<Self> {
        let mut cx = Dec::new(b); cx.var32()?;
        Ok(Self { a: cx.var32()?, b: cx.var32()? })
    }
}

impl UnloadChunk {
    pub fn encode(&self) -> Bytes {
        let mut cx = Enc::new(); cx.var32(ID_UNLOAD_CHUNK);
        cx.i32(self.b); cx.i32(self.a); cx.finish()
    }
    pub fn decode(b: Bytes) -> R<Self> {
        let mut cx = Dec::new(b); cx.var32()?;
        let b = cx.i32()?; let a = cx.i32()?;
        Ok(Self { a, b })
    }
}

impl SetTime {
    pub fn encode(&self) -> Bytes {
        let mut cx = Enc::new(); cx.var32(ID_SET_TIME);
        cx.i64(self.a); cx.i64(self.b); cx.bool(self.c); cx.finish()
    }
    pub fn decode(b: Bytes) -> R<Self> {
        let mut cx = Dec::new(b); cx.var32()?;
        Ok(Self { a: cx.i64()?, b: cx.i64()?, c: cx.bool()? })
    }
}
