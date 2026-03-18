use bytes::{BufMut, BytesMut};
use crate::error::{Err, R};

#[inline] pub fn ru8(b: &mut &[u8]) -> R<u8> {
    if b.is_empty() { return Err(Err::Eof); }
    let v = b[0]; *b = &b[1..]; Ok(v)
}

#[inline] pub fn rbool(b: &mut &[u8]) -> R<bool> { Ok(ru8(b)? != 0) }

#[inline] pub fn ri8(b: &mut &[u8]) -> R<i8> { ru8(b).map(|n| n as i8) }

#[inline] pub fn ru16(b: &mut &[u8]) -> R<u16> {
    if b.len() < 2 { return Err(Err::Eof); }
    let v = u16::from_le_bytes([b[0], b[1]]); *b = &b[2..]; Ok(v)
}

#[inline] pub fn ri16(b: &mut &[u8]) -> R<i16> { ru16(b).map(|n| n as i16) }

#[inline] pub fn ru32(b: &mut &[u8]) -> R<u32> {
    if b.len() < 4 { return Err(Err::Eof); }
    let v = u32::from_le_bytes([b[0], b[1], b[2], b[3]]); *b = &b[4..]; Ok(v)
}

#[inline] pub fn ri32(b: &mut &[u8]) -> R<i32> { ru32(b).map(|n| n as i32) }

#[inline] pub fn ru64(b: &mut &[u8]) -> R<u64> {
    if b.len() < 8 { return Err(Err::Eof); }
    let v = u64::from_le_bytes(b[..8].try_into().unwrap()); *b = &b[8..]; Ok(v)
}

#[inline] pub fn ri64(b: &mut &[u8]) -> R<i64> { ru64(b).map(|n| n as i64) }

#[inline] pub fn rf32(b: &mut &[u8]) -> R<f32> { ru32(b).map(f32::from_bits) }

#[inline] pub fn rf64(b: &mut &[u8]) -> R<f64> { ru64(b).map(f64::from_bits) }

pub fn rvarint(b: &mut &[u8]) -> R<u32> {
    let (mut n, mut s) = (0u32, 0u32);
    loop {
        let x = ru8(b)?;
        n |= ((x & 0x7F) as u32) << s;
        if x & 0x80 == 0 { return Ok(n); }
        s += 7;
        if s >= 35 { return Err(Err::Eof); }
    }
}

pub fn rvarinti(b: &mut &[u8]) -> R<i32> {
    let n = rvarint(b)?;
    Ok(((n >> 1) as i32) ^ -((n & 1) as i32))
}

pub fn rvarint64(b: &mut &[u8]) -> R<u64> {
    let (mut n, mut s) = (0u64, 0u64);
    loop {
        let x = ru8(b)?;
        n |= ((x & 0x7F) as u64) << s;
        if x & 0x80 == 0 { return Ok(n); }
        s += 7;
        if s >= 70 { return Err(Err::Eof); }
    }
}

pub fn rvarinti64(b: &mut &[u8]) -> R<i64> {
    let n = rvarint64(b)?;
    Ok(((n >> 1) as i64) ^ -((n & 1) as i64))
}

pub fn rstr(b: &mut &[u8]) -> R<String> {
    let n = rvarint(b)? as usize;
    if b.len() < n { return Err(Err::Eof); }
    let s = std::str::from_utf8(&b[..n])
        .map_err(|e| Err::Nbt(e.to_string()))?
        .to_owned();
    *b = &b[n..];
    Ok(s)
}

pub fn rbytes(b: &mut &[u8], n: usize) -> R<Vec<u8>> {
    if b.len() < n { return Err(Err::Eof); }
    let v = b[..n].to_vec();
    *b = &b[n..];
    Ok(v)
}

#[inline] pub fn wu8(out: &mut BytesMut, v: u8)   { out.put_u8(v); }
#[inline] pub fn wbool(out: &mut BytesMut, v: bool) { out.put_u8(v as u8); }
#[inline] pub fn wi8(out: &mut BytesMut, v: i8)   { out.put_i8(v); }
#[inline] pub fn wu16(out: &mut BytesMut, v: u16)  { out.extend_from_slice(&v.to_le_bytes()); }
#[inline] pub fn wi16(out: &mut BytesMut, v: i16)  { wu16(out, v as u16); }
#[inline] pub fn wu32(out: &mut BytesMut, v: u32)  { out.extend_from_slice(&v.to_le_bytes()); }
#[inline] pub fn wi32(out: &mut BytesMut, v: i32)  { wu32(out, v as u32); }
#[inline] pub fn wu64(out: &mut BytesMut, v: u64)  { out.extend_from_slice(&v.to_le_bytes()); }
#[inline] pub fn wi64(out: &mut BytesMut, v: i64)  { wu64(out, v as u64); }
#[inline] pub fn wf32(out: &mut BytesMut, v: f32)  { wu32(out, v.to_bits()); }
#[inline] pub fn wf64(out: &mut BytesMut, v: f64)  { wu64(out, v.to_bits()); }

pub fn wvarint(out: &mut BytesMut, mut v: u32) {
    loop {
        let x = (v & 0x7F) as u8;
        v >>= 7;
        out.put_u8(if v != 0 { x | 0x80 } else { x });
        if v == 0 { break; }
    }
}

pub fn wvarinti(out: &mut BytesMut, v: i32) {
    wvarint(out, ((v << 1) ^ (v >> 31)) as u32);
}

pub fn wvarint64(out: &mut BytesMut, mut v: u64) {
    loop {
        let x = (v & 0x7F) as u8;
        v >>= 7;
        out.put_u8(if v != 0 { x | 0x80 } else { x });
        if v == 0 { break; }
    }
}

pub fn wvarinti64(out: &mut BytesMut, v: i64) {
    wvarint64(out, ((v << 1) ^ (v >> 63)) as u64);
}

pub fn wstr(out: &mut BytesMut, s: &str) {
    wvarint(out, s.len() as u32);
    out.extend_from_slice(s.as_bytes());
}
