use bytes::{BufMut, Bytes, BytesMut};
use crate::error::{Err, R};

pub const OFFLINE_MSG: [u8; 16] = [
    0x00, 0xFF, 0xFF, 0x00, 0xFE, 0xFE, 0xFE, 0xFE,
    0xFD, 0xFD, 0xFD, 0xFD, 0x12, 0x34, 0x56, 0x78,
];

pub const ID_PING:       u8 = 0x01;
pub const ID_PONG:       u8 = 0x1C;
pub const ID_OPEN_REQ1:  u8 = 0x05;
pub const ID_OPEN_REP1:  u8 = 0x06;
pub const ID_OPEN_REQ2:  u8 = 0x07;
pub const ID_OPEN_REP2:  u8 = 0x08;
pub const ID_INCOMPATIBLE: u8 = 0x19;
pub const RAKNET_VER:    u8 = 11;

pub struct Ping { pub a: u64 }
pub struct Pong { pub a: u64, pub b: u64, pub c: String }
pub struct OpenReq1 { pub a: u16 }
pub struct OpenRep1 { pub a: u64, pub b: bool, pub c: u16 }
pub struct OpenReq2 { pub a: u64, pub b: u16, pub c: u64 }
pub struct OpenRep2 { pub a: u64, pub b: u64, pub c: u16 }

impl Ping {
    pub fn encode(&self) -> Bytes {
        let mut o = BytesMut::new();
        o.put_u8(ID_PING);
        o.extend_from_slice(&self.a.to_be_bytes());
        o.extend_from_slice(&OFFLINE_MSG);
        o.freeze()
    }

    pub fn decode(b: &[u8]) -> R<Self> {
        if b.len() < 25 { return Err(Err::Eof); }
        let a = u64::from_be_bytes(b[1..9].try_into().unwrap());
        Ok(Self { a })
    }
}

impl Pong {
    pub fn encode(&self) -> Bytes {
        let mut o = BytesMut::new();
        o.put_u8(ID_PONG);
        o.extend_from_slice(&self.a.to_be_bytes());
        o.extend_from_slice(&self.b.to_be_bytes());
        o.extend_from_slice(&OFFLINE_MSG);
        let id_bytes = self.c.as_bytes();
        o.extend_from_slice(&(id_bytes.len() as u16).to_be_bytes());
        o.extend_from_slice(id_bytes);
        o.freeze()
    }

    pub fn decode(b: &[u8]) -> R<Self> {
        if b.len() < 35 { return Err(Err::Eof); }
        let a = u64::from_be_bytes(b[1..9].try_into().unwrap());
        let bb = u64::from_be_bytes(b[9..17].try_into().unwrap());
        let slen = u16::from_be_bytes(b[33..35].try_into().unwrap()) as usize;
        if b.len() < 35 + slen { return Err(Err::Eof); }
        let c = String::from_utf8_lossy(&b[35..35 + slen]).into_owned();
        Ok(Self { a, b: bb, c })
    }
}

impl OpenReq1 {
    pub fn encode(&self) -> Bytes {
        let mut o = BytesMut::new();
        o.put_u8(ID_OPEN_REQ1);
        o.extend_from_slice(&OFFLINE_MSG);
        o.put_u8(RAKNET_VER);
        let pad = (self.a as usize).saturating_sub(46);
        o.extend_from_slice(&vec![0u8; pad]);
        o.freeze()
    }

    pub fn decode(b: &[u8]) -> R<Self> {
        if b.len() < 18 { return Err(Err::Eof); }
        Ok(Self { a: b.len() as u16 })
    }
}

impl OpenRep1 {
    pub fn encode(&self) -> Bytes {
        let mut o = BytesMut::new();
        o.put_u8(ID_OPEN_REP1);
        o.extend_from_slice(&OFFLINE_MSG);
        o.extend_from_slice(&self.a.to_be_bytes());
        o.put_u8(if self.b { 1 } else { 0 });
        o.extend_from_slice(&self.c.to_be_bytes());
        o.freeze()
    }

    pub fn decode(b: &[u8]) -> R<Self> {
        if b.len() < 28 { return Err(Err::Eof); }
        let a = u64::from_be_bytes(b[17..25].try_into().unwrap());
        let bb = b[25] != 0;
        let c = u16::from_be_bytes(b[26..28].try_into().unwrap());
        Ok(Self { a, b: bb, c })
    }
}

impl OpenReq2 {
    pub fn encode(&self) -> Bytes {
        let mut o = BytesMut::new();
        o.put_u8(ID_OPEN_REQ2);
        o.extend_from_slice(&OFFLINE_MSG);
        o.extend_from_slice(&self.a.to_be_bytes());
        o.extend_from_slice(&self.b.to_be_bytes());
        o.extend_from_slice(&self.c.to_be_bytes());
        o.freeze()
    }

    pub fn decode(b: &[u8]) -> R<Self> {
        if b.len() < 34 { return Err(Err::Eof); }
        let a = u64::from_be_bytes(b[17..25].try_into().unwrap());
        let bb = u16::from_be_bytes(b[25..27].try_into().unwrap());
        let c = u64::from_be_bytes(b[27..35].try_into().unwrap());
        Ok(Self { a, b: bb, c })
    }
}

impl OpenRep2 {
    pub fn encode(&self) -> Bytes {
        let mut o = BytesMut::new();
        o.put_u8(ID_OPEN_REP2);
        o.extend_from_slice(&OFFLINE_MSG);
        o.extend_from_slice(&self.a.to_be_bytes());
        o.extend_from_slice(&self.b.to_be_bytes());
        o.extend_from_slice(&self.c.to_be_bytes());
        o.put_u8(0);
        o.freeze()
    }

    pub fn decode(b: &[u8]) -> R<Self> {
        if b.len() < 35 { return Err(Err::Eof); }
        let a = u64::from_be_bytes(b[17..25].try_into().unwrap());
        let bb = u64::from_be_bytes(b[25..33].try_into().unwrap());
        let c = u16::from_be_bytes(b[33..35].try_into().unwrap());
        Ok(Self { a, b: bb, c })
    }
}
