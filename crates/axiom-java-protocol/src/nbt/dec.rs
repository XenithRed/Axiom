use std::collections::HashMap;
use crate::error::{Err, R};
use super::tag::Tag;

const MAX_DEPTH: usize = 512;

pub fn decode(b: &[u8]) -> R<(String, Tag)> {
    let mut c = b;
    read_named(&mut c, 0)
}

pub fn decode_network(b: &[u8]) -> R<Tag> {
    let mut c = b;
    let tid = ru8(&mut c)?;
    read_payload(&mut c, tid, 0)
}

fn read_named(c: &mut &[u8], d: usize) -> R<(String, Tag)> {
    if d > MAX_DEPTH { return Err(Err::Nbt("depth exceeded".into())); }
    let tid = ru8(c)?;
    if tid == 0 { return Ok((String::new(), Tag::End)); }
    let name = read_str(c)?;
    let tag  = read_payload(c, tid, d)?;
    Ok((name, tag))
}

fn read_payload(c: &mut &[u8], tid: u8, d: usize) -> R<Tag> {
    match tid {
        0  => Ok(Tag::End),
        1  => Ok(Tag::Byte(ri8(c)?)),
        2  => Ok(Tag::Short(ri16(c)?)),
        3  => Ok(Tag::Int(ri32(c)?)),
        4  => Ok(Tag::Long(ri64(c)?)),
        5  => Ok(Tag::Float(f32::from_bits(ru32(c)?))),
        6  => Ok(Tag::Double(f64::from_bits(ru64(c)?))),
        7  => {
            let n = ri32(c)? as usize;
            need(c, n)?;
            let v: Vec<i8> = c[..n].iter().map(|&b| b as i8).collect();
            *c = &c[n..]; Ok(Tag::ByteArray(v))
        }
        8  => Ok(Tag::String(read_str(c)?)),
        9  => {
            let lt = ru8(c)?;
            let n  = ri32(c)? as usize;
            let mut v = Vec::with_capacity(n);
            for _ in 0..n { v.push(read_payload(c, lt, d + 1)?); }
            Ok(Tag::List(v))
        }
        10 => {
            let mut m = HashMap::new();
            loop {
                let (name, tag) = read_named(c, d + 1)?;
                if matches!(tag, Tag::End) { break; }
                m.insert(name, tag);
            }
            Ok(Tag::Compound(m))
        }
        11 => {
            let n = ri32(c)? as usize;
            let mut v = Vec::with_capacity(n);
            for _ in 0..n { v.push(ri32(c)?); }
            Ok(Tag::IntArray(v))
        }
        12 => {
            let n = ri32(c)? as usize;
            let mut v = Vec::with_capacity(n);
            for _ in 0..n { v.push(ri64(c)?); }
            Ok(Tag::LongArray(v))
        }
        x  => Err(Err::Nbt(format!("unknown tag {x}"))),
    }
}

fn need(c: &[u8], n: usize) -> R<()> { if c.len() < n { Err(Err::Eof) } else { Ok(()) } }

fn ru8(c: &mut &[u8])  -> R<u8>  { need(c, 1)?; let v = c[0]; *c = &c[1..]; Ok(v) }
fn ri8(c: &mut &[u8])  -> R<i8>  { ru8(c).map(|n| n as i8) }
fn ru16(c: &mut &[u8]) -> R<u16> { need(c,2)?; let v = u16::from_be_bytes([c[0],c[1]]); *c=&c[2..]; Ok(v) }
fn ri16(c: &mut &[u8]) -> R<i16> { ru16(c).map(|n| n as i16) }
fn ru32(c: &mut &[u8]) -> R<u32> { need(c,4)?; let v = u32::from_be_bytes(c[..4].try_into().unwrap()); *c=&c[4..]; Ok(v) }
fn ri32(c: &mut &[u8]) -> R<i32> { ru32(c).map(|n| n as i32) }
fn ru64(c: &mut &[u8]) -> R<u64> { need(c,8)?; let v = u64::from_be_bytes(c[..8].try_into().unwrap()); *c=&c[8..]; Ok(v) }
fn ri64(c: &mut &[u8]) -> R<i64> { ru64(c).map(|n| n as i64) }

fn read_str(c: &mut &[u8]) -> R<String> {
    let n = ru16(c)? as usize;
    need(c, n)?;
    let s = String::from_utf8(c[..n].to_vec())
        .map_err(|e| Err::Nbt(e.to_string()))?;
    *c = &c[n..]; Ok(s)
}
