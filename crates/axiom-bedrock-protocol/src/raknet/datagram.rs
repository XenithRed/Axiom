use bytes::{Buf, BufMut, Bytes, BytesMut};
use crate::error::{Err, R};
use super::{ack::U24, frame::Frame};

pub const DATA_MIN: u8 = 0x80;
pub const DATA_MAX: u8 = 0x8F;
pub const MTU:     usize = 1400;

#[derive(Debug, Clone)]
pub struct Datagram {
    pub a: U24,
    pub b: Vec<Frame>,
}

impl Datagram {
    pub fn new(seq: U24) -> Self {
        Self { a: seq, b: Vec::new() }
    }

    pub fn push(&mut self, f: Frame) {
        self.b.push(f);
    }

    pub fn encode(&self) -> Bytes {
        let mut out = BytesMut::new();
        out.put_u8(DATA_MIN);
        self.a.write(&mut out);
        for f in &self.b {
            f.encode(&mut out);
        }
        out.freeze()
    }

    pub fn decode(raw: &[u8]) -> R<Self> {
        if raw.len() < 4 { return Err(Err::Eof); }
        let id = raw[0];
        if !(DATA_MIN..=DATA_MAX).contains(&id) { return Err(Err::BadId(id)); }
        let mut cur = &raw[1..];
        let seq = U24::read(&mut cur)?;
        let mut frames = Vec::new();
        while !cur.is_empty() {
            frames.push(Frame::decode(&mut cur)?);
        }
        Ok(Self { a: seq, b: frames })
    }

    pub fn is_data(id: u8) -> bool { (DATA_MIN..=DATA_MAX).contains(&id) }
}
