use bytes::Bytes;
use crate::codec::{Dec, Enc};
use crate::error::R;

pub const ID_DISCONNECT:     i32 = 0x00;
pub const ID_ENC_REQUEST:    i32 = 0x01;
pub const ID_LOGIN_SUCCESS:  i32 = 0x02;
pub const ID_SET_COMPRESS:   i32 = 0x03;
pub const ID_PLUGIN_REQUEST: i32 = 0x04;
pub const ID_COOKIE_REQUEST: i32 = 0x05;

#[derive(Debug, Clone)]
pub struct Disconnect { pub a: String }

#[derive(Debug, Clone)]
pub struct EncRequest {
    pub a: String,
    pub b: Bytes,
    pub c: Bytes,
    pub d: bool,
}

#[derive(Debug, Clone)]
pub struct LoginSuccess {
    pub a: u128,
    pub b: String,
    pub c: Vec<Property>,
    pub d: bool,
}

#[derive(Debug, Clone)]
pub struct Property {
    pub a: String,
    pub b: String,
    pub c: Option<String>,
}

#[derive(Debug, Clone)]
pub struct SetCompress { pub a: i32 }

#[derive(Debug, Clone)]
pub struct PluginRequest {
    pub a: i32,
    pub b: String,
    pub c: Bytes,
}

impl Disconnect {
    pub fn encode(&self) -> Bytes {
        let mut cx = Enc::new();
        cx.var32(ID_DISCONNECT);
        cx.str(&self.a);
        cx.finish()
    }
    pub fn decode(b: Bytes) -> R<Self> {
        let mut cx = Dec::new(b); cx.var32()?;
        Ok(Self { a: cx.str()? })
    }
}

impl EncRequest {
    pub fn encode(&self) -> Bytes {
        let mut cx = Enc::new();
        cx.var32(ID_ENC_REQUEST);
        cx.str(&self.a);
        cx.bytes_prefixed(&self.b);
        cx.bytes_prefixed(&self.c);
        cx.bool(self.d);
        cx.finish()
    }
    pub fn decode(b: Bytes) -> R<Self> {
        let mut cx = Dec::new(b); cx.var32()?;
        Ok(Self {
            a: cx.str()?,
            b: cx.bytes_prefixed()?,
            c: cx.bytes_prefixed()?,
            d: cx.bool()?,
        })
    }
}

impl LoginSuccess {
    pub fn encode(&self) -> Bytes {
        let mut cx = Enc::new();
        cx.var32(ID_LOGIN_SUCCESS);
        cx.uuid(self.a);
        cx.str(&self.b);
        cx.var32(self.c.len() as i32);
        for p in &self.c {
            cx.str(&p.a);
            cx.str(&p.b);
            cx.bool(p.c.is_some());
            if let Some(ref s) = p.c { cx.str(s); }
        }
        cx.bool(self.d);
        cx.finish()
    }
    pub fn decode(b: Bytes) -> R<Self> {
        let mut cx = Dec::new(b); cx.var32()?;
        let a = cx.uuid()?;
        let bb = cx.str()?;
        let n = cx.var32()? as usize;
        let mut c = Vec::with_capacity(n);
        for _ in 0..n {
            let pa = cx.str()?;
            let pb = cx.str()?;
            let has = cx.bool()?;
            let pc = if has { Some(cx.str()?) } else { None };
            c.push(Property { a: pa, b: pb, c: pc });
        }
        let d = cx.bool()?;
        Ok(Self { a, b: bb, c, d })
    }
}

impl SetCompress {
    pub fn encode(&self) -> Bytes {
        let mut cx = Enc::new();
        cx.var32(ID_SET_COMPRESS);
        cx.var32(self.a);
        cx.finish()
    }
    pub fn decode(b: Bytes) -> R<Self> {
        let mut cx = Dec::new(b); cx.var32()?;
        Ok(Self { a: cx.var32()? })
    }
}
