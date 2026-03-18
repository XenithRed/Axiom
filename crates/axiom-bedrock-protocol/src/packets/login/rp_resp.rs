use bytes::Bytes;
use crate::codec::{Dec, Enc};
use crate::error::R;

pub const ID: u8 = 0x08;

#[derive(Debug, Clone, PartialEq, Eq)]
#[repr(u8)]
pub enum Status {
    Refused     = 1,
    SendPacks   = 2,
    HaveAll     = 3,
    Completed   = 4,
}

impl Status {
    pub fn from_u8(n: u8) -> Option<Self> {
        match n {
            1 => Some(Self::Refused),
            2 => Some(Self::SendPacks),
            3 => Some(Self::HaveAll),
            4 => Some(Self::Completed),
            _ => None,
        }
    }
}

#[derive(Debug, Clone)]
pub struct RpResp {
    pub a: Status,
    pub b: Vec<String>,
}

impl RpResp {
    pub fn encode(&self) -> Bytes {
        let mut cx = Enc::new();
        cx.u8(ID);
        cx.u8(self.a.clone() as u8);
        cx.u16(self.b.len() as u16);
        for s in &self.b { cx.str(s); }
        cx.finish()
    }

    pub fn decode(b: Bytes) -> R<Self> {
        let mut cx = Dec::new(b);
        cx.u8()?;
        let a = Status::from_u8(cx.u8()?).unwrap_or(Status::Refused);
        let n = cx.u16()? as usize;
        let mut packs = Vec::with_capacity(n);
        for _ in 0..n { packs.push(cx.str()?); }
        Ok(Self { a, b: packs })
    }
}
