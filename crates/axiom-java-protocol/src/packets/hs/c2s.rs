use bytes::Bytes;
use crate::codec::{Dec, Enc};
use crate::error::R;

pub const ID: i32 = 0x00;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(i32)]
pub enum NextState { Status = 1, Login = 2, Transfer = 3 }

impl NextState {
    pub fn from_i32(n: i32) -> Self {
        match n { 2 => Self::Login, 3 => Self::Transfer, _ => Self::Status }
    }
}

#[derive(Debug, Clone)]
pub struct Handshake {
    pub a: i32,
    pub b: String,
    pub c: u16,
    pub d: NextState,
}

impl Handshake {
    pub fn encode(&self) -> Bytes {
        let mut cx = Enc::new();
        cx.var32(ID);
        cx.var32(self.a);
        cx.str(&self.b);
        cx.u16(self.c);
        cx.var32(self.d as i32);
        cx.finish()
    }

    pub fn decode(b: Bytes) -> R<Self> {
        let mut cx = Dec::new(b);
        cx.var32()?;
        Ok(Self {
            a: cx.var32()?,
            b: cx.str_max(255)?,
            c: cx.u16()?,
            d: NextState::from_i32(cx.var32()?),
        })
    }
}
