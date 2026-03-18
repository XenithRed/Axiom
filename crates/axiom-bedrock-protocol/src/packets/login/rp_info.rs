use bytes::Bytes;
use crate::codec::{Dec, Enc};
use crate::error::R;

pub const ID: u8 = 0x06;

#[derive(Debug, Clone)]
pub struct PackInfo {
    pub a: String,
    pub b: String,
    pub c: u64,
    pub d: bool,
    pub e: String,
    pub f: String,
    pub g: bool,
}

#[derive(Debug, Clone)]
pub struct RpInfo {
    pub a: bool,
    pub b: bool,
    pub c: bool,
    pub d: Vec<PackInfo>,
    pub e: Vec<PackInfo>,
}

impl RpInfo {
    pub fn empty() -> Self {
        Self { a: false, b: false, c: false, d: vec![], e: vec![] }
    }

    pub fn encode(&self) -> Bytes {
        let mut cx = Enc::new();
        cx.u8(ID);
        cx.bool(self.a);
        cx.bool(self.b);
        cx.bool(self.c);
        cx.u16(self.d.len() as u16);
        for p in &self.d { write_pack(&mut cx, p); }
        cx.u16(self.e.len() as u16);
        for p in &self.e { write_pack(&mut cx, p); }
        cx.finish()
    }

    pub fn decode(b: Bytes) -> R<Self> {
        let mut cx = Dec::new(b);
        cx.u8()?;
        let a = cx.bool()?;
        let b = cx.bool()?;
        let c = cx.bool()?;
        let dn = cx.u16()? as usize;
        let mut d = Vec::with_capacity(dn);
        for _ in 0..dn { d.push(read_pack(&mut cx)?); }
        let en = cx.u16()? as usize;
        let mut e = Vec::with_capacity(en);
        for _ in 0..en { e.push(read_pack(&mut cx)?); }
        Ok(Self { a, b, c, d, e })
    }
}

fn write_pack(cx: &mut Enc, p: &PackInfo) {
    cx.str(&p.a);
    cx.str(&p.b);
    cx.u64(p.c);
    cx.bool(p.d);
    cx.str(&p.e);
    cx.str(&p.f);
    cx.bool(p.g);
}

fn read_pack(cx: &mut Dec) -> R<PackInfo> {
    Ok(PackInfo {
        a: cx.str()?,
        b: cx.str()?,
        c: cx.u64()?,
        d: cx.bool()?,
        e: cx.str()?,
        f: cx.str()?,
        g: cx.bool()?,
    })
}
