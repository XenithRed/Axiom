use bytes::{Bytes, BytesMut};
use crate::error::R;
use super::le::*;

pub struct Enc {
    a: BytesMut,
}

impl Enc {
    pub fn new() -> Self { Self { a: BytesMut::with_capacity(256) } }
    pub fn with_cap(n: usize) -> Self { Self { a: BytesMut::with_capacity(n) } }

    pub fn u8(&mut self, v: u8)     { wu8(&mut self.a, v); }
    pub fn bool(&mut self, v: bool) { wbool(&mut self.a, v); }
    pub fn i8(&mut self, v: i8)     { wi8(&mut self.a, v); }
    pub fn u16(&mut self, v: u16)   { wu16(&mut self.a, v); }
    pub fn i16(&mut self, v: i16)   { wi16(&mut self.a, v); }
    pub fn u32(&mut self, v: u32)   { wu32(&mut self.a, v); }
    pub fn i32(&mut self, v: i32)   { wi32(&mut self.a, v); }
    pub fn u64(&mut self, v: u64)   { wu64(&mut self.a, v); }
    pub fn i64(&mut self, v: i64)   { wi64(&mut self.a, v); }
    pub fn f32(&mut self, v: f32)   { wf32(&mut self.a, v); }
    pub fn f64(&mut self, v: f64)   { wf64(&mut self.a, v); }
    pub fn varint(&mut self, v: u32)  { wvarint(&mut self.a, v); }
    pub fn varinti(&mut self, v: i32) { wvarinti(&mut self.a, v); }
    pub fn varint64(&mut self, v: u64)  { wvarint64(&mut self.a, v); }
    pub fn varinti64(&mut self, v: i64) { wvarinti64(&mut self.a, v); }
    pub fn str(&mut self, s: &str)  { wstr(&mut self.a, s); }
    pub fn bytes(&mut self, b: &[u8]) { self.a.extend_from_slice(b); }
    pub fn len(&self) -> usize { self.a.len() }
    pub fn finish(self) -> Bytes { self.a.freeze() }
}

impl Default for Enc { fn default() -> Self { Self::new() } }
