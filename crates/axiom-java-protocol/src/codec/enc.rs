use bytes::{BufMut, Bytes, BytesMut};
use crate::varint::{write_var32, write_var64, var32_len};

pub struct Enc {
    a: BytesMut,
}

impl Enc {
    pub fn new() -> Self { Self { a: BytesMut::with_capacity(256) } }
    pub fn with_cap(n: usize) -> Self { Self { a: BytesMut::with_capacity(n) } }

    pub fn u8(&mut self, v: u8)    { self.a.put_u8(v); }
    pub fn i8(&mut self, v: i8)    { self.a.put_i8(v); }
    pub fn bool(&mut self, v: bool) { self.a.put_u8(v as u8); }
    pub fn u16(&mut self, v: u16)  { self.a.put_u16(v); }
    pub fn i16(&mut self, v: i16)  { self.a.put_i16(v); }
    pub fn u32(&mut self, v: u32)  { self.a.put_u32(v); }
    pub fn i32(&mut self, v: i32)  { self.a.put_i32(v); }
    pub fn u64(&mut self, v: u64)  { self.a.put_u64(v); }
    pub fn i64(&mut self, v: i64)  { self.a.put_i64(v); }
    pub fn f32(&mut self, v: f32)  { self.a.put_f32(v); }
    pub fn f64(&mut self, v: f64)  { self.a.put_f64(v); }
    pub fn u128(&mut self, v: u128) { self.a.put_u128(v); }

    pub fn var32(&mut self, v: i32) { write_var32(&mut self.a, v); }
    pub fn var64(&mut self, v: i64) { write_var64(&mut self.a, v); }

    pub fn str(&mut self, s: &str) {
        self.var32(s.len() as i32);
        self.a.extend_from_slice(s.as_bytes());
    }

    pub fn str_max(&mut self, s: &str, _max: usize) {
        self.str(s);
    }

    pub fn bytes(&mut self, b: &[u8]) { self.a.extend_from_slice(b); }

    pub fn bytes_prefixed(&mut self, b: &[u8]) {
        self.var32(b.len() as i32);
        self.a.extend_from_slice(b);
    }

    pub fn uuid(&mut self, u: u128) { self.u128(u); }

    pub fn pos(&mut self, x: i32, y: i16, z: i32) {
        let v = ((x as i64 & 0x3FFFFFF) << 38)
            | ((z as i64 & 0x3FFFFFF) << 12)
            | (y as i64 & 0xFFF);
        self.i64(v);
    }

    pub fn angle(&mut self, deg: f32) {
        self.u8((deg / 360.0 * 256.0) as u8);
    }

    pub fn len(&self) -> usize { self.a.len() }

    pub fn finish(self) -> Bytes { self.a.freeze() }

    pub fn finish_with_id(mut self, id: i32) -> Bytes {
        let inner = self.a.freeze();
        let mut out = BytesMut::with_capacity(var32_len(id) + inner.len());
        write_var32(&mut out, id);
        out.extend_from_slice(&inner);
        out.freeze()
    }
}

impl Default for Enc { fn default() -> Self { Self::new() } }
