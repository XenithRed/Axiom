use bytes::Bytes;
use crate::error::{Err, R};
use super::le::*;

pub struct Dec {
    a: Bytes,
    b: usize,
}

impl Dec {
    pub fn new(b: Bytes) -> Self { Self { a: b, b: 0 } }
    pub fn from_slice(s: &[u8]) -> Self { Self::new(Bytes::copy_from_slice(s)) }
    pub fn remaining(&self) -> usize { self.a.len() - self.b }
    pub fn is_empty(&self) -> bool { self.b >= self.a.len() }
    pub fn pos(&self) -> usize { self.b }

    fn cur(&mut self) -> &[u8] { &self.a[self.b..] }

    fn adv(&mut self, n: usize) { self.b += n; }

    fn need(&self, n: usize) -> R<()> {
        if self.remaining() < n { Err(Err::Eof) } else { Ok(()) }
    }

    pub fn u8(&mut self) -> R<u8> {
        self.need(1)?;
        let mut s = self.cur();
        let v = ru8(&mut s)?;
        self.adv(1);
        Ok(v)
    }

    pub fn bool(&mut self) -> R<bool> { self.u8().map(|n| n != 0) }
    pub fn i8(&mut self)  -> R<i8>  { self.u8().map(|n| n as i8) }

    pub fn u16(&mut self) -> R<u16> {
        self.need(2)?;
        let mut s = self.cur();
        let v = ru16(&mut s)?;
        self.adv(2);
        Ok(v)
    }

    pub fn i16(&mut self) -> R<i16> { self.u16().map(|n| n as i16) }

    pub fn u32(&mut self) -> R<u32> {
        self.need(4)?;
        let mut s = self.cur();
        let v = ru32(&mut s)?;
        self.adv(4);
        Ok(v)
    }

    pub fn i32(&mut self) -> R<i32> { self.u32().map(|n| n as i32) }

    pub fn u64(&mut self) -> R<u64> {
        self.need(8)?;
        let mut s = self.cur();
        let v = ru64(&mut s)?;
        self.adv(8);
        Ok(v)
    }

    pub fn i64(&mut self) -> R<i64> { self.u64().map(|n| n as i64) }
    pub fn f32(&mut self) -> R<f32> { self.u32().map(f32::from_bits) }
    pub fn f64(&mut self) -> R<f64> { self.u64().map(f64::from_bits) }

    pub fn varint(&mut self) -> R<u32> {
        let mut s = self.cur();
        let old = s.len();
        let v = rvarint(&mut s)?;
        self.adv(old - s.len());
        Ok(v)
    }

    pub fn varinti(&mut self) -> R<i32> {
        let n = self.varint()?;
        Ok(((n >> 1) as i32) ^ -((n & 1) as i32))
    }

    pub fn varint64(&mut self) -> R<u64> {
        let mut s = self.cur();
        let old = s.len();
        let v = rvarint64(&mut s)?;
        self.adv(old - s.len());
        Ok(v)
    }

    pub fn varinti64(&mut self) -> R<i64> {
        let n = self.varint64()?;
        Ok(((n >> 1) as i64) ^ -((n & 1) as i64))
    }

    pub fn str(&mut self) -> R<String> {
        let n = self.varint()? as usize;
        self.need(n)?;
        let s = std::str::from_utf8(&self.a[self.b..self.b + n])
            .map_err(|e| Err::Nbt(e.to_string()))?
            .to_owned();
        self.adv(n);
        Ok(s)
    }

    pub fn slice(&mut self, n: usize) -> R<Bytes> {
        self.need(n)?;
        let s = self.a.slice(self.b..self.b + n);
        self.adv(n);
        Ok(s)
    }

    pub fn rest(&mut self) -> Bytes {
        let s = self.a.slice(self.b..);
        self.b = self.a.len();
        s
    }
}
