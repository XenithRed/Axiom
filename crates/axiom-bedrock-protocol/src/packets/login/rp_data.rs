use bytes::Bytes;
use crate::codec::{Dec, Enc};
use crate::error::R;

pub const ID_INFO:  u8 = 0x52;
pub const ID_CHUNK: u8 = 0x53;
pub const ID_STACK: u8 = 0x07;

#[derive(Debug, Clone)]
pub struct RpDataInfo {
    pub a: String,
    pub b: u32,
    pub c: u64,
    pub d: u32,
}

#[derive(Debug, Clone)]
pub struct RpChunk {
    pub a: String,
    pub b: u32,
    pub c: Bytes,
}

#[derive(Debug, Clone)]
pub struct RpStack {
    pub a: bool,
    pub b: Vec<StackEntry>,
    pub c: Vec<StackEntry>,
    pub d: bool,
    pub e: String,
    pub f: String,
}

#[derive(Debug, Clone)]
pub struct StackEntry {
    pub a: String,
    pub b: String,
    pub c: bool,
}

impl RpDataInfo {
    pub fn encode(&self) -> Bytes {
        let mut cx = Enc::new();
        cx.u8(ID_INFO);
        cx.str(&self.a);
        cx.u32(self.b);
        cx.u64(self.c);
        cx.u32(self.d);
        cx.finish()
    }

    pub fn decode(b: Bytes) -> R<Self> {
        let mut cx = Dec::new(b);
        cx.u8()?;
        Ok(Self {
            a: cx.str()?,
            b: cx.u32()?,
            c: cx.u64()?,
            d: cx.u32()?,
        })
    }
}

impl RpChunk {
    pub fn encode(&self) -> Bytes {
        let mut cx = Enc::new();
        cx.u8(ID_CHUNK);
        cx.str(&self.a);
        cx.u32(self.b);
        cx.u32(self.c.len() as u32);
        cx.bytes(&self.c);
        cx.finish()
    }

    pub fn decode(b: Bytes) -> R<Self> {
        let mut cx = Dec::new(b);
        cx.u8()?;
        let a = cx.str()?;
        let bb = cx.u32()?;
        let n  = cx.u32()? as usize;
        let c  = cx.slice(n)?;
        Ok(Self { a, b: bb, c })
    }
}

impl RpStack {
    pub fn empty() -> Self {
        Self { a: false, b: vec![], c: vec![], d: false, e: String::new(), f: String::new() }
    }

    pub fn encode(&self) -> Bytes {
        let mut cx = Enc::new();
        cx.u8(ID_STACK);
        cx.bool(self.a);
        cx.u16(self.b.len() as u16);
        for e in &self.b { write_entry(&mut cx, e); }
        cx.u16(self.c.len() as u16);
        for e in &self.c { write_entry(&mut cx, e); }
        cx.bool(self.d);
        cx.str(&self.e);
        cx.str(&self.f);
        cx.finish()
    }

    pub fn decode(b: Bytes) -> R<Self> {
        let mut cx = Dec::new(b);
        cx.u8()?;
        let a  = cx.bool()?;
        let bn = cx.u16()? as usize;
        let mut bv = Vec::with_capacity(bn);
        for _ in 0..bn { bv.push(read_entry(&mut cx)?); }
        let cn = cx.u16()? as usize;
        let mut cv = Vec::with_capacity(cn);
        for _ in 0..cn { cv.push(read_entry(&mut cx)?); }
        let d = cx.bool()?;
        let e = cx.str()?;
        let f = cx.str()?;
        Ok(Self { a, b: bv, c: cv, d, e, f })
    }
}

fn write_entry(cx: &mut Enc, e: &StackEntry) {
    cx.str(&e.a);
    cx.str(&e.b);
    cx.bool(e.c);
}

fn read_entry(cx: &mut Dec) -> R<StackEntry> {
    Ok(StackEntry { a: cx.str()?, b: cx.str()?, c: cx.bool()? })
}
