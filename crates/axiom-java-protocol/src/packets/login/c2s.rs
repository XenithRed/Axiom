use bytes::Bytes;
use crate::codec::{Dec, Enc};
use crate::error::R;

pub const ID_LOGIN_START:   i32 = 0x00;
pub const ID_ENC_RESPONSE:  i32 = 0x01;
pub const ID_PLUGIN_RESPONSE: i32 = 0x02;
pub const ID_LOGIN_ACK:     i32 = 0x03;
pub const ID_COOKIE_RESPONSE: i32 = 0x04;

#[derive(Debug, Clone)]
pub struct LoginStart {
    pub a: String,
    pub b: u128,
}

#[derive(Debug, Clone)]
pub struct EncResponse {
    pub a: Bytes,
    pub b: Bytes,
}

#[derive(Debug, Clone)]
pub struct PluginResponse {
    pub a: i32,
    pub b: Option<Bytes>,
}

#[derive(Debug, Clone)]
pub struct LoginAck;

impl LoginStart {
    pub fn encode(&self) -> Bytes {
        let mut cx = Enc::new();
        cx.var32(ID_LOGIN_START);
        cx.str(&self.a);
        cx.uuid(self.b);
        cx.finish()
    }
    pub fn decode(b: Bytes) -> R<Self> {
        let mut cx = Dec::new(b); cx.var32()?;
        Ok(Self { a: cx.str_max(16)?, b: cx.uuid()? })
    }
}

impl EncResponse {
    pub fn encode(&self) -> Bytes {
        let mut cx = Enc::new();
        cx.var32(ID_ENC_RESPONSE);
        cx.bytes_prefixed(&self.a);
        cx.bytes_prefixed(&self.b);
        cx.finish()
    }
    pub fn decode(b: Bytes) -> R<Self> {
        let mut cx = Dec::new(b); cx.var32()?;
        Ok(Self { a: cx.bytes_prefixed()?, b: cx.bytes_prefixed()? })
    }
}

impl PluginResponse {
    pub fn encode(&self) -> Bytes {
        let mut cx = Enc::new();
        cx.var32(ID_PLUGIN_RESPONSE);
        cx.var32(self.a);
        cx.bool(self.b.is_some());
        if let Some(ref d) = self.b { cx.bytes(d); }
        cx.finish()
    }
    pub fn decode(b: Bytes) -> R<Self> {
        let mut cx = Dec::new(b); cx.var32()?;
        let a = cx.var32()?;
        let has = cx.bool()?;
        let bb = if has { Some(cx.rest()) } else { None };
        Ok(Self { a, b: bb })
    }
}

impl LoginAck {
    pub fn encode() -> Bytes {
        let mut cx = Enc::new();
        cx.var32(ID_LOGIN_ACK);
        cx.finish()
    }
}
