use bytes::Bytes;
use crate::codec::{Dec, Enc};
use crate::error::R;

pub const ID_CHAT:         i32 = 0x06;
pub const ID_CMD:          i32 = 0x04;
pub const ID_CMD_SIGNED:   i32 = 0x05;
pub const ID_CHAT_SESSION: i32 = 0x06;

#[derive(Debug, Clone)]
pub struct Chat {
    pub a: String,
    pub b: i64,
    pub c: Option<Bytes>,
}

#[derive(Debug, Clone)]
pub struct Command {
    pub a: String,
    pub b: i64,
}

impl Chat {
    pub fn encode(&self) -> Bytes {
        let mut cx = Enc::new(); cx.var32(ID_CHAT);
        cx.str_max(&self.a, 256);
        cx.i64(self.b);
        cx.bool(self.c.is_some());
        if let Some(ref s) = self.c { cx.bytes(s); }
        cx.finish()
    }
    pub fn decode(b: Bytes) -> R<Self> {
        let mut cx = Dec::new(b); cx.var32()?;
        let a = cx.str_max(256)?;
        let bb = cx.i64()?;
        let has = cx.bool()?;
        let c = if has { Some(cx.rest()) } else { None };
        Ok(Self { a, b: bb, c })
    }
}

impl Command {
    pub fn encode(&self) -> Bytes {
        let mut cx = Enc::new(); cx.var32(ID_CMD);
        cx.str_max(&self.a, 256);
        cx.i64(self.b);
        cx.finish()
    }
    pub fn decode(b: Bytes) -> R<Self> {
        let mut cx = Dec::new(b); cx.var32()?;
        Ok(Self { a: cx.str_max(256)?, b: cx.i64()? })
    }
}
