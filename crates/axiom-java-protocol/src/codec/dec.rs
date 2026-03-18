use bytes::{Buf, Bytes};
use crate::error::{Err, R};
use crate::varint::{read_var32_bytes, read_var32, read_var64};

pub struct Dec {
    a: Bytes,
    b: usize,
}

impl Dec {
    pub fn new(b: Bytes) -> Self { Self { a: b, b: 0 } }
    pub fn from_slice(s: &[u8]) -> Self { Self::new(Bytes::copy_from_slice(s)) }
    pub fn remaining(&self) -> usize { self.a.len() - self.b }
    pub fn is_empty(&self) -> bool { self.b >= self.a.len() }

    fn need(&self, n: usize) -> R<()> {
        if self.remaining() < n { Err(Err::Eof) } else { Ok(()) }
    }

    fn adv(&mut self, n: usize) { self.b += n; }

    fn cur(&self) -> &[u8] { &self.a[self.b..] }

    pub fn u8(&mut self) -> R<u8> {
        self.need(1)?;
        let v = self.a[self.b]; self.adv(1); Ok(v)
    }

    pub fn i8(&mut self)  -> R<i8>  { self.u8().map(|n| n as i8) }
    pub fn bool(&mut self) -> R<bool> { self.u8().map(|n| n != 0) }

    pub fn u16(&mut self) -> R<u16> {
        self.need(2)?;
        let v = u16::from_be_bytes([self.a[self.b], self.a[self.b+1]]);
        self.adv(2); Ok(v)
    }

    pub fn i16(&mut self) -> R<i16> { self.u16().map(|n| n as i16) }

    pub fn u32(&mut self) -> R<u32> {
        self.need(4)?;
        let v = u32::from_be_bytes(self.a[self.b..self.b+4].try_into().unwrap());
        self.adv(4); Ok(v)
    }

    pub fn i32(&mut self) -> R<i32> { self.u32().map(|n| n as i32) }

    pub fn u64(&mut self) -> R<u64> {
        self.need(8)?;
        let v = u64::from_be_bytes(self.a[self.b..self.b+8].try_into().unwrap());
        self.adv(8); Ok(v)
    }

    pub fn i64(&mut self) -> R<i64> { self.u64().map(|n| n as i64) }

    pub fn f32(&mut self) -> R<f32> { self.u32().map(f32::from_bits) }
    pub fn f64(&mut self) -> R<f64> { self.u64().map(f64::from_bits) }

    pub fn u128(&mut self) -> R<u128> {
        self.need(16)?;
        let v = u128::from_be_bytes(self.a[self.b..self.b+16].try_into().unwrap());
        self.adv(16); Ok(v)
    }

    pub fn var32(&mut self) -> R<i32> {
        let mut s = self.cur();
        let old = s.len();
        let v = read_var32(&mut s)?;
        self.adv(old - s.len());
        Ok(v)
    }

    pub fn var64(&mut self) -> R<i64> {
        let mut s = self.cur();
        let old = s.len();
        let v = read_var64(&mut s)?;
        self.adv(old - s.len());
        Ok(v)
    }

    pub fn str(&mut self) -> R<String> {
        let n = self.var32()? as usize;
        self.need(n)?;
        let v = String::from_utf8(self.a[self.b..self.b+n].to_vec())?;
        self.adv(n); Ok(v)
    }

    pub fn str_max(&mut self, max: usize) -> R<String> {
        let s = self.str()?;
        if s.len() > max { return Err(Err::StrTooLong(s.len(), max)); }
        Ok(s)
    }

    pub fn uuid(&mut self) -> R<u128> { self.u128() }

    pub fn slice(&mut self, n: usize) -> R<Bytes> {
        self.need(n)?;
        let s = self.a.slice(self.b..self.b+n);
        self.adv(n); Ok(s)
    }

    pub fn bytes_prefixed(&mut self) -> R<Bytes> {
        let n = self.var32()? as usize;
        self.slice(n)
    }

    pub fn rest(&mut self) -> Bytes {
        let s = self.a.slice(self.b..);
        self.b = self.a.len(); s
    }

    pub fn pos(&mut self) -> R<(i32, i16, i32)> {
        let v = self.i64()?;
        let x = (v >> 38) as i32;
        let z = (v << 26 >> 38) as i32;
        let y = (v << 52 >> 52) as i16;
        Ok((x, y, z))
    }

    pub fn angle(&mut self) -> R<f32> {
        Ok(self.u8()? as f32 / 256.0 * 360.0)
    }
}
