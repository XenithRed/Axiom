use bytes::{Buf, BufMut, Bytes, BytesMut};
use crate::error::{Err, R};

pub const MAX_VAR32: usize = 5;
pub const MAX_VAR64: usize = 10;

#[inline]
pub fn read_var32(b: &mut &[u8]) -> R<i32> {
    let (mut n, mut s) = (0u32, 0u32);
    loop {
        if b.is_empty() { return Err(Err::Eof); }
        let x = b[0]; *b = &b[1..];
        n |= ((x & 0x7F) as u32) << s;
        if x & 0x80 == 0 { return Ok(n as i32); }
        s += 7;
        if s >= 35 { return Err(Err::VarOverflow); }
    }
}

#[inline]
pub fn read_var64(b: &mut &[u8]) -> R<i64> {
    let (mut n, mut s) = (0u64, 0u64);
    loop {
        if b.is_empty() { return Err(Err::Eof); }
        let x = b[0]; *b = &b[1..];
        n |= ((x & 0x7F) as u64) << s;
        if x & 0x80 == 0 { return Ok(n as i64); }
        s += 7;
        if s >= 70 { return Err(Err::VarOverflow); }
    }
}

#[inline]
pub fn write_var32(out: &mut BytesMut, mut v: i32) {
    let mut u = v as u32;
    loop {
        let x = (u & 0x7F) as u8;
        u >>= 7;
        out.put_u8(if u != 0 { x | 0x80 } else { x });
        if u == 0 { break; }
    }
}

#[inline]
pub fn write_var64(out: &mut BytesMut, v: i64) {
    let mut u = v as u64;
    loop {
        let x = (u & 0x7F) as u8;
        u >>= 7;
        out.put_u8(if u != 0 { x | 0x80 } else { x });
        if u == 0 { break; }
    }
}

#[inline]
pub fn var32_len(v: i32) -> usize {
    let u = v as u32;
    match u {
        0x00000000..=0x0000007F => 1,
        0x00000080..=0x00003FFF => 2,
        0x00004000..=0x001FFFFF => 3,
        0x00200000..=0x0FFFFFFF => 4,
        _ => 5,
    }
}

pub fn read_var32_bytes(b: &mut Bytes) -> R<i32> {
    let (mut n, mut s) = (0u32, 0u32);
    loop {
        if b.is_empty() { return Err(Err::Eof); }
        let x = b.get_u8();
        n |= ((x & 0x7F) as u32) << s;
        if x & 0x80 == 0 { return Ok(n as i32); }
        s += 7;
        if s >= 35 { return Err(Err::VarOverflow); }
    }
}
