use bytes::Bytes;
use crate::codec::{Dec, Enc};
use crate::error::R;

pub const ID_COOKIE_RESPONSE:  i32 = 0x00;
pub const ID_PLUGIN_MSG:       i32 = 0x01;
pub const ID_ACK_FINISH_CFG:   i32 = 0x02;
pub const ID_KEEP_ALIVE:       i32 = 0x03;
pub const ID_PONG:             i32 = 0x04;
pub const ID_RPACK_RESPONSE:   i32 = 0x05;
pub const ID_KNOWN_PACKS:      i32 = 0x06;

#[derive(Debug, Clone)]
pub struct PluginMsg { pub a: String, pub b: Option<Bytes> }

#[derive(Debug, Clone)]
pub struct AckFinishCfg;

#[derive(Debug, Clone)]
pub struct KeepAlive { pub a: i64 }

#[derive(Debug, Clone)]
pub struct KnownPack { pub a: String, pub b: String, pub c: String }

#[derive(Debug, Clone)]
pub struct KnownPacks { pub a: Vec<KnownPack> }

impl PluginMsg {
    pub fn encode(&self) -> Bytes {
        let mut cx = Enc::new(); cx.var32(ID_PLUGIN_MSG);
        cx.str(&self.a);
        cx.bool(self.b.is_some());
        if let Some(ref d) = self.b { cx.bytes(d); }
        cx.finish()
    }
    pub fn decode(b: Bytes) -> R<Self> {
        let mut cx = Dec::new(b); cx.var32()?;
        let a = cx.str()?;
        let has = cx.bool()?;
        let bb = if has { Some(cx.rest()) } else { None };
        Ok(Self { a, b: bb })
    }
}

impl AckFinishCfg {
    pub fn encode() -> Bytes {
        let mut cx = Enc::new(); cx.var32(ID_ACK_FINISH_CFG); cx.finish()
    }
}

impl KeepAlive {
    pub fn encode(&self) -> Bytes {
        let mut cx = Enc::new(); cx.var32(ID_KEEP_ALIVE);
        cx.i64(self.a); cx.finish()
    }
    pub fn decode(b: Bytes) -> R<Self> {
        let mut cx = Dec::new(b); cx.var32()?;
        Ok(Self { a: cx.i64()? })
    }
}

impl KnownPacks {
    pub fn encode(&self) -> Bytes {
        let mut cx = Enc::new(); cx.var32(ID_KNOWN_PACKS);
        cx.var32(self.a.len() as i32);
        for p in &self.a { cx.str(&p.a); cx.str(&p.b); cx.str(&p.c); }
        cx.finish()
    }
    pub fn decode(b: Bytes) -> R<Self> {
        let mut cx = Dec::new(b); cx.var32()?;
        let n = cx.var32()? as usize;
        let mut v = Vec::with_capacity(n);
        for _ in 0..n {
            v.push(KnownPack { a: cx.str()?, b: cx.str()?, c: cx.str()? });
        }
        Ok(Self { a: v })
    }
}
