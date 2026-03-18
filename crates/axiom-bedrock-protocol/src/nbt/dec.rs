use std::collections::HashMap;
use crate::error::{Err, R};
use super::tag::Tag;

const MAX_DEPTH: usize = 512;

pub fn decode(b: &[u8]) -> R<(String, Tag)> {
    let mut cur = b;
    read_named(&mut cur, 0)
}

fn read_named<'a>(cur: &mut &'a [u8], depth: usize) -> R<(String, Tag)> {
    if depth > MAX_DEPTH { return Err(Err::Nbt("depth exceeded".into())); }
    let tid  = read_u8(cur)?;
    if tid == 0 { return Ok((String::new(), Tag::End)); }
    let name = read_str(cur)?;
    let tag  = read_payload(cur, tid, depth)?;
    Ok((name, tag))
}

fn read_payload(cur: &mut &[u8], tid: u8, depth: usize) -> R<Tag> {
    match tid {
        0  => Ok(Tag::End),
        1  => Ok(Tag::Byte(read_i8(cur)?)),
        2  => Ok(Tag::Short(read_i16(cur)?)),
        3  => Ok(Tag::Int(read_i32(cur)?)),
        4  => Ok(Tag::Long(read_i64(cur)?)),
        5  => Ok(Tag::Float(f32::from_bits(read_u32(cur)?))),
        6  => Ok(Tag::Double(f64::from_bits(read_u64(cur)?))),
        7  => {
            let n = read_i32(cur)? as usize;
            need(cur, n)?;
            let v: Vec<i8> = cur[..n].iter().map(|&b| b as i8).collect();
            *cur = &cur[n..];
            Ok(Tag::ByteArray(v))
        }
        8  => Ok(Tag::String(read_str(cur)?)),
        9  => read_list(cur, depth),
        10 => read_compound(cur, depth),
        11 => {
            let n = read_i32(cur)? as usize;
            let mut v = Vec::with_capacity(n);
            for _ in 0..n { v.push(read_i32(cur)?); }
            Ok(Tag::IntArray(v))
        }
        12 => {
            let n = read_i32(cur)? as usize;
            let mut v = Vec::with_capacity(n);
            for _ in 0..n { v.push(read_i64(cur)?); }
            Ok(Tag::LongArray(v))
        }
        x  => Err(Err::Nbt(format!("unknown tag id {x}"))),
    }
}

fn read_list(cur: &mut &[u8], depth: usize) -> R<Tag> {
    let tid = read_u8(cur)?;
    let n   = read_i32(cur)? as usize;
    let mut v = Vec::with_capacity(n);
    for _ in 0..n {
        v.push(read_payload(cur, tid, depth + 1)?);
    }
    Ok(Tag::List(v))
}

fn read_compound(cur: &mut &[u8], depth: usize) -> R<Tag> {
    let mut m = HashMap::new();
    loop {
        let (name, tag) = read_named(cur, depth + 1)?;
        if matches!(tag, Tag::End) { break; }
        m.insert(name, tag);
    }
    Ok(Tag::Compound(m))
}

fn need(cur: &[u8], n: usize) -> R<()> {
    if cur.len() < n { Err(Err::Eof) } else { Ok(()) }
}

fn read_u8(cur: &mut &[u8]) -> R<u8> {
    need(cur, 1)?;
    let v = cur[0]; *cur = &cur[1..]; Ok(v)
}

fn read_i8(cur: &mut &[u8])  -> R<i8>  { read_u8(cur).map(|n| n as i8) }

fn read_u16(cur: &mut &[u8]) -> R<u16> {
    need(cur, 2)?;
    let v = u16::from_le_bytes([cur[0], cur[1]]); *cur = &cur[2..]; Ok(v)
}

fn read_i16(cur: &mut &[u8]) -> R<i16> { read_u16(cur).map(|n| n as i16) }

fn read_u32(cur: &mut &[u8]) -> R<u32> {
    need(cur, 4)?;
    let v = u32::from_le_bytes([cur[0], cur[1], cur[2], cur[3]]); *cur = &cur[4..]; Ok(v)
}

fn read_i32(cur: &mut &[u8]) -> R<i32> { read_u32(cur).map(|n| n as i32) }

fn read_u64(cur: &mut &[u8]) -> R<u64> {
    need(cur, 8)?;
    let v = u64::from_le_bytes(cur[..8].try_into().unwrap()); *cur = &cur[8..]; Ok(v)
}

fn read_i64(cur: &mut &[u8]) -> R<i64> { read_u64(cur).map(|n| n as i64) }

fn read_str(cur: &mut &[u8]) -> R<String> {
    let n = read_u16(cur)? as usize;
    need(cur, n)?;
    let s = std::str::from_utf8(&cur[..n])
        .map_err(|e| Err::Nbt(e.to_string()))?
        .to_owned();
    *cur = &cur[n..];
    Ok(s)
}
