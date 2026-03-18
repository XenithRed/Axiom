use bytes::Bytes;
use crate::codec::{Dec, Enc};
use crate::error::R;

pub const ID_SYSTEM_CHAT:    i32 = 0x6C;
pub const ID_PLAYER_CHAT:    i32 = 0x39;
pub const ID_DISGUISED_CHAT: i32 = 0x1E;

#[derive(Debug, Clone)]
pub struct SystemChat { pub a: String, pub b: bool }

#[derive(Debug, Clone)]
pub struct PlayerChat {
    pub a: u128,
    pub b: i32,
    pub c: Option<Bytes>,
    pub d: String,
    pub e: i64,
    pub f: Option<String>,
    pub g: i32,
}

#[derive(Debug, Clone)]
pub struct DisguisedChat { pub a: String, pub b: i32, pub c: Option<String>, pub d: Option<String> }

impl SystemChat {
    pub fn encode(&self) -> Bytes {
        let mut cx = Enc::new(); cx.var32(ID_SYSTEM_CHAT);
        cx.str(&self.a); cx.bool(self.b); cx.finish()
    }
    pub fn decode(b: Bytes) -> R<Self> {
        let mut cx = Dec::new(b); cx.var32()?;
        Ok(Self { a: cx.str()?, b: cx.bool()? })
    }
}

impl PlayerChat {
    pub fn encode(&self) -> Bytes {
        let mut cx = Enc::new(); cx.var32(ID_PLAYER_CHAT);
        cx.uuid(self.a); cx.var32(self.b);
        cx.bool(self.c.is_some());
        if let Some(ref s) = self.c { cx.bytes(s); }
        cx.str(&self.d); cx.i64(self.e);
        cx.bool(self.f.is_some());
        if let Some(ref s) = self.f { cx.str(s); }
        cx.var32(self.g);
        cx.finish()
    }
    pub fn decode(b: Bytes) -> R<Self> {
        let mut cx = Dec::new(b); cx.var32()?;
        let a = cx.uuid()?; let bb = cx.var32()?;
        let has_sig = cx.bool()?;
        let c = if has_sig { Some(cx.slice(256)?) } else { None };
        let d = cx.str()?; let e = cx.i64()?;
        let has_un = cx.bool()?;
        let f = if has_un { Some(cx.str()?) } else { None };
        let g = cx.var32()?;
        Ok(Self { a, b: bb, c, d, e, f, g })
    }
}

impl DisguisedChat {
    pub fn encode(&self) -> Bytes {
        let mut cx = Enc::new(); cx.var32(ID_DISGUISED_CHAT);
        cx.str(&self.a); cx.var32(self.b);
        cx.bool(self.c.is_some());
        if let Some(ref s) = self.c { cx.str(s); }
        cx.bool(self.d.is_some());
        if let Some(ref s) = self.d { cx.str(s); }
        cx.finish()
    }
    pub fn decode(b: Bytes) -> R<Self> {
        let mut cx = Dec::new(b); cx.var32()?;
        let a = cx.str()?; let bb = cx.var32()?;
        let hc = cx.bool()?; let c = if hc { Some(cx.str()?) } else { None };
        let hd = cx.bool()?; let d = if hd { Some(cx.str()?) } else { None };
        Ok(Self { a, b: bb, c, d })
    }
}
