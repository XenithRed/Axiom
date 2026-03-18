use bytes::{BufMut, Bytes, BytesMut};
use crate::error::{Err, R};

pub const ID_CONN_REQ:    u8 = 0x09;
pub const ID_CONN_REPLY:  u8 = 0x10;
pub const ID_NEW_CONN:    u8 = 0x13;
pub const ID_DISCONN:     u8 = 0x15;

pub struct ConnReq {
    pub a: u64,
    pub b: u64,
    pub c: bool,
}

pub struct ConnReply {
    pub a: u64,
    pub b: u64,
    pub c: u64,
    pub d: u16,
}

pub struct NewConn {
    pub a: u64,
    pub b: u64,
}

pub struct Disconn;

impl ConnReq {
    pub fn encode(&self) -> Bytes {
        let mut o = BytesMut::new();
        o.put_u8(ID_CONN_REQ);
        o.extend_from_slice(&self.a.to_be_bytes());
        o.extend_from_slice(&self.b.to_be_bytes());
        o.put_u8(if self.c { 1 } else { 0 });
        o.freeze()
    }

    pub fn decode(b: &[u8]) -> R<Self> {
        if b.len() < 17 { return Err(Err::Eof); }
        let a = u64::from_be_bytes(b[1..9].try_into().unwrap());
        let bb = u64::from_be_bytes(b[9..17].try_into().unwrap());
        let c = b.get(17).copied().unwrap_or(0) != 0;
        Ok(Self { a, b: bb, c })
    }
}

impl ConnReply {
    pub fn encode(&self) -> Bytes {
        let mut o = BytesMut::new();
        o.put_u8(ID_CONN_REPLY);
        o.extend_from_slice(&self.a.to_be_bytes());
        o.put_u8(0);
        o.extend_from_slice(&self.d.to_be_bytes());
        o.extend_from_slice(&[0u8; 10]);
        o.extend_from_slice(&self.b.to_be_bytes());
        o.extend_from_slice(&self.c.to_be_bytes());
        o.freeze()
    }

    pub fn decode(b: &[u8]) -> R<Self> {
        if b.len() < 29 { return Err(Err::Eof); }
        let a  = u64::from_be_bytes(b[1..9].try_into().unwrap());
        let d  = u16::from_be_bytes(b[10..12].try_into().unwrap());
        let bb = u64::from_be_bytes(b[22..30].try_into().unwrap());
        let c  = if b.len() >= 38 {
            u64::from_be_bytes(b[30..38].try_into().unwrap())
        } else { 0 };
        Ok(Self { a, b: bb, c, d })
    }
}

impl NewConn {
    pub fn encode(&self) -> Bytes {
        let mut o = BytesMut::new();
        o.put_u8(ID_NEW_CONN);
        o.extend_from_slice(&[0u8; 4]);
        o.extend_from_slice(&self.a.to_be_bytes());
        o.extend_from_slice(&self.b.to_be_bytes());
        o.freeze()
    }

    pub fn decode(b: &[u8]) -> R<Self> {
        if b.len() < 21 { return Err(Err::Eof); }
        let a = u64::from_be_bytes(b[5..13].try_into().unwrap());
        let bb = u64::from_be_bytes(b[13..21].try_into().unwrap());
        Ok(Self { a, b: bb })
    }
}

impl Disconn {
    pub fn encode() -> Bytes {
        Bytes::from_static(&[ID_DISCONN])
    }
}
