use bytes::Bytes;
use crate::codec::{Dec, Enc};
use crate::error::R;

pub const ID_COOKIE_REQUEST:   i32 = 0x00;
pub const ID_PLUGIN_MSG:       i32 = 0x01;
pub const ID_DISCONNECT:       i32 = 0x02;
pub const ID_FINISH_CFG:       i32 = 0x03;
pub const ID_KEEP_ALIVE:       i32 = 0x04;
pub const ID_PING:             i32 = 0x05;
pub const ID_RESET_CHAT:       i32 = 0x06;
pub const ID_REGISTRY_DATA:    i32 = 0x07;
pub const ID_REMOVE_RPACK:     i32 = 0x08;
pub const ID_ADD_RPACK:        i32 = 0x09;
pub const ID_STORE_COOKIE:     i32 = 0x0A;
pub const ID_TRANSFER:         i32 = 0x0B;
pub const ID_FEATURE_FLAGS:    i32 = 0x0C;
pub const ID_UPDATE_TAGS:      i32 = 0x0D;
pub const ID_KNOWN_PACKS:      i32 = 0x0E;
pub const ID_CUSTOM_REPORT:    i32 = 0x0F;

#[derive(Debug, Clone)]
pub struct PluginMsg { pub a: String, pub b: Bytes }

#[derive(Debug, Clone)]
pub struct Disconnect { pub a: String }

#[derive(Debug, Clone)]
pub struct FinishCfg;

#[derive(Debug, Clone)]
pub struct KeepAlive { pub a: i64 }

#[derive(Debug, Clone)]
pub struct RegistryData { pub a: String, pub b: Bytes }

#[derive(Debug, Clone)]
pub struct KnownPack { pub a: String, pub b: String, pub c: String }

#[derive(Debug, Clone)]
pub struct KnownPacks { pub a: Vec<KnownPack> }

#[derive(Debug, Clone)]
pub struct FeatureFlags { pub a: Vec<String> }

impl PluginMsg {
    pub fn encode(&self) -> Bytes {
        let mut cx = Enc::new(); cx.var32(ID_PLUGIN_MSG);
        cx.str(&self.a); cx.bytes(&self.b); cx.finish()
    }
    pub fn decode(b: Bytes) -> R<Self> {
        let mut cx = Dec::new(b); cx.var32()?;
        Ok(Self { a: cx.str()?, b: cx.rest() })
    }
}

impl Disconnect {
    pub fn encode(&self) -> Bytes {
        let mut cx = Enc::new(); cx.var32(ID_DISCONNECT);
        cx.str(&self.a); cx.finish()
    }
    pub fn decode(b: Bytes) -> R<Self> {
        let mut cx = Dec::new(b); cx.var32()?;
        Ok(Self { a: cx.str()? })
    }
}

impl FinishCfg {
    pub fn encode() -> Bytes {
        let mut cx = Enc::new(); cx.var32(ID_FINISH_CFG); cx.finish()
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

impl FeatureFlags {
    pub fn encode(&self) -> Bytes {
        let mut cx = Enc::new(); cx.var32(ID_FEATURE_FLAGS);
        cx.var32(self.a.len() as i32);
        for f in &self.a { cx.str(f); }
        cx.finish()
    }
    pub fn decode(b: Bytes) -> R<Self> {
        let mut cx = Dec::new(b); cx.var32()?;
        let n = cx.var32()? as usize;
        let mut v = Vec::with_capacity(n);
        for _ in 0..n { v.push(cx.str()?); }
        Ok(Self { a: v })
    }
}
