use bytes::Bytes;
use crate::codec::{Dec, Enc};
use crate::error::R;

pub const ID_INV_TRANS: u8 = 0x1E;
pub const ID_ITEM_USE:  u8 = 0x1F;
pub const ID_CONTAINER_CLOSE: u8 = 0x2F;

#[derive(Debug, Clone)]
pub struct InvTrans {
    pub a: u32,
    pub b: Vec<Action>,
    pub c: bool,
}

#[derive(Debug, Clone)]
pub struct Action {
    pub a: u32,
    pub b: i16,
    pub c: u8,
    pub d: i16,
    pub e: u8,
}

#[derive(Debug, Clone)]
pub struct ItemUse {
    pub a: u32,
    pub b: i32, pub c: i32, pub d: i32,
    pub e: i32,
    pub f: u64,
    pub g: f32, pub h: f32, pub i: f32,
    pub j: f32, pub k: f32, pub l: f32,
    pub m: bool,
}

#[derive(Debug, Clone)]
pub struct ContainerClose {
    pub a: i8,
    pub b: bool,
}

impl InvTrans {
    pub fn encode(&self) -> Bytes {
        let mut cx = Enc::new();
        cx.u8(ID_INV_TRANS);
        cx.u32(self.a);
        cx.varint(self.b.len() as u32);
        for ac in &self.b {
            cx.varint(ac.a);
            cx.i16(ac.b); cx.u8(ac.c);
            cx.i16(ac.d); cx.u8(ac.e);
        }
        cx.bool(self.c);
        cx.finish()
    }

    pub fn decode(b: Bytes) -> R<Self> {
        let mut cx = Dec::new(b);
        cx.u8()?;
        let a = cx.u32()?;
        let n = cx.varint()? as usize;
        let mut bv = Vec::with_capacity(n);
        for _ in 0..n {
            bv.push(Action {
                a: cx.varint()?,
                b: cx.i16()?, c: cx.u8()?,
                d: cx.i16()?, e: cx.u8()?,
            });
        }
        let c = cx.bool()?;
        Ok(Self { a, b: bv, c })
    }
}

impl ItemUse {
    pub fn encode(&self) -> Bytes {
        let mut cx = Enc::new();
        cx.u8(ID_ITEM_USE);
        cx.varint(self.a);
        cx.varinti(self.b); cx.varinti(self.c); cx.varinti(self.d);
        cx.varinti(self.e);
        cx.varint64(self.f);
        cx.f32(self.g); cx.f32(self.h); cx.f32(self.i);
        cx.f32(self.j); cx.f32(self.k); cx.f32(self.l);
        cx.bool(self.m);
        cx.finish()
    }

    pub fn decode(b: Bytes) -> R<Self> {
        let mut cx = Dec::new(b);
        cx.u8()?;
        Ok(Self {
            a: cx.varint()?,
            b: cx.varinti()?, c: cx.varinti()?, d: cx.varinti()?,
            e: cx.varinti()?,
            f: cx.varint64()?,
            g: cx.f32()?, h: cx.f32()?, i: cx.f32()?,
            j: cx.f32()?, k: cx.f32()?, l: cx.f32()?,
            m: cx.bool()?,
        })
    }
}

impl ContainerClose {
    pub fn encode(&self) -> Bytes {
        let mut cx = Enc::new();
        cx.u8(ID_CONTAINER_CLOSE);
        cx.i8(self.a);
        cx.bool(self.b);
        cx.finish()
    }

    pub fn decode(b: Bytes) -> R<Self> {
        let mut cx = Dec::new(b);
        cx.u8()?;
        Ok(Self { a: cx.i8()?, b: cx.bool()? })
    }
}
