use bytes::{BufMut, Bytes, BytesMut};
use crate::error::{Err, R};

#[derive(Debug, Clone)]
pub enum Val {
    Byte(i8),
    Int(i32),
    Float(f32),
    Str(String),
    Bool(bool),
    OptUuid(Option<[u8; 16]>),
}

#[derive(Debug, Clone)]
pub struct Entry { pub a: u8, pub b: Val }

pub fn java_to_bedrock(entries: &[Entry]) -> Vec<Entry> {
    entries.iter().filter_map(|e| remap_entry(e)).collect()
}

fn remap_entry(e: &Entry) -> Option<Entry> {
    let (idx, val) = match e.a {
        0 => (0, remap_flags(&e.b)?),
        1 => return None,
        2 => (2, e.b.clone()),
        3 => (3, e.b.clone()),
        7 => return None,
        8 => (7, e.b.clone()),
        9 => (9, e.b.clone()),
        _ => (e.a, e.b.clone()),
    };
    Some(Entry { a: idx, b: val })
}

fn remap_flags(v: &Val) -> Option<Val> {
    if let Val::Byte(n) = v {
        let mut out = 0i8;
        if n & 0x01 != 0 { out |= 0x01; }
        if n & 0x02 != 0 { out |= 0x10; }
        if n & 0x04 != 0 { out |= 0x20; }
        if n & 0x08 != 0 { out |= 0x40; }
        if n & 0x10 != 0 { out |= 0x04; }
        if n & 0x20 != 0 { out |= 0x02; }
        Some(Val::Byte(out))
    } else { None }
}

pub fn decode_java(raw: &[u8]) -> R<Vec<Entry>> {
    let mut out = Vec::new();
    let mut c = raw;
    loop {
        if c.is_empty() { break; }
        let idx = c[0]; c = &c[1..];
        if idx == 0xFF { break; }
        let type_id = read_var32(&mut c)?;
        let val = match type_id {
            0 => Val::Byte(c[0] as i8),
            1 => { let v = read_var32(&mut c)? as i32; Val::Int(v) }
            2 => { let v = read_f32(&mut c)?; Val::Float(v) }
            3 => { let s = read_str(&mut c)?; Val::Str(s) }
            7 => { let b = c[0] != 0; c = &c[1..]; Val::Bool(b) }
            _ => { break; }
        };
        out.push(Entry { a: idx, b: val });
    }
    Ok(out)
}

pub fn encode_bedrock(entries: &[Entry]) -> Bytes {
    let mut o = BytesMut::new();
    for e in entries {
        o.put_u8(e.a);
        match &e.b {
            Val::Byte(n)    => { o.put_u8(0); o.put_i8(*n); }
            Val::Int(n)     => { o.put_u8(1); write_var32(&mut o, *n as u32); }
            Val::Float(n)   => { o.put_u8(2); o.extend_from_slice(&n.to_le_bytes()); }
            Val::Str(s)     => { o.put_u8(3); write_str(&mut o, s); }
            Val::Bool(b)    => { o.put_u8(4); o.put_u8(*b as u8); }
            Val::OptUuid(u) => {
                o.put_u8(5);
                o.put_u8(u.is_some() as u8);
                if let Some(uid) = u { o.extend_from_slice(uid); }
            }
        }
    }
    o.put_u8(0x7F);
    o.freeze()
}

fn read_var32(c: &mut &[u8]) -> R<u32> {
    let (mut n, mut s) = (0u32, 0u32);
    loop {
        if c.is_empty() { return Err(Err::Nbt("eof in varint".into())); }
        let x = c[0]; *c = &c[1..];
        n |= ((x & 0x7F) as u32) << s;
        if x & 0x80 == 0 { return Ok(n); }
        s += 7;
    }
}

fn read_f32(c: &mut &[u8]) -> R<f32> {
    if c.len() < 4 { return Err(Err::Nbt("eof".into())); }
    let v = f32::from_bits(u32::from_be_bytes([c[0],c[1],c[2],c[3]]));
    *c = &c[4..]; Ok(v)
}

fn read_str(c: &mut &[u8]) -> R<String> {
    let n = read_var32(c)? as usize;
    if c.len() < n { return Err(Err::Nbt("eof in str".into())); }
    let s = String::from_utf8_lossy(&c[..n]).into_owned();
    *c = &c[n..]; Ok(s)
}

fn write_var32(o: &mut BytesMut, mut v: u32) {
    loop {
        let x = (v & 0x7F) as u8;
        v >>= 7;
        o.put_u8(if v != 0 { x | 0x80 } else { x });
        if v == 0 { break; }
    }
}

fn write_str(o: &mut BytesMut, s: &str) {
    write_var32(o, s.len() as u32);
    o.extend_from_slice(s.as_bytes());
}
