use bytes::{BufMut, BytesMut};
use super::tag::Tag;
use std::collections::HashMap;

pub fn encode(name: &str, tag: &Tag) -> Vec<u8> {
    let mut out = BytesMut::new();
    write_named(&mut out, name, tag);
    out.to_vec()
}

fn write_named(out: &mut BytesMut, name: &str, tag: &Tag) {
    out.put_u8(tag.type_id());
    write_str(out, name);
    write_payload(out, tag);
}

fn write_payload(out: &mut BytesMut, tag: &Tag) {
    match tag {
        Tag::End             => {}
        Tag::Byte(n)         => out.put_i8(*n),
        Tag::Short(n)        => out.extend_from_slice(&n.to_le_bytes()),
        Tag::Int(n)          => out.extend_from_slice(&n.to_le_bytes()),
        Tag::Long(n)         => out.extend_from_slice(&n.to_le_bytes()),
        Tag::Float(n)        => out.extend_from_slice(&n.to_bits().to_le_bytes()),
        Tag::Double(n)       => out.extend_from_slice(&n.to_bits().to_le_bytes()),
        Tag::ByteArray(v)    => {
            out.extend_from_slice(&(v.len() as i32).to_le_bytes());
            for b in v { out.put_i8(*b); }
        }
        Tag::String(s)       => write_str(out, s),
        Tag::List(v)         => write_list(out, v),
        Tag::Compound(m)     => write_compound(out, m),
        Tag::IntArray(v)     => {
            out.extend_from_slice(&(v.len() as i32).to_le_bytes());
            for n in v { out.extend_from_slice(&n.to_le_bytes()); }
        }
        Tag::LongArray(v)    => {
            out.extend_from_slice(&(v.len() as i32).to_le_bytes());
            for n in v { out.extend_from_slice(&n.to_le_bytes()); }
        }
    }
}

fn write_list(out: &mut BytesMut, v: &[Tag]) {
    let tid = v.first().map(|t| t.type_id()).unwrap_or(0);
    out.put_u8(tid);
    out.extend_from_slice(&(v.len() as i32).to_le_bytes());
    for t in v { write_payload(out, t); }
}

fn write_compound(out: &mut BytesMut, m: &HashMap<String, Tag>) {
    for (k, v) in m {
        write_named(out, k, v);
    }
    out.put_u8(0);
}

fn write_str(out: &mut BytesMut, s: &str) {
    out.extend_from_slice(&(s.len() as u16).to_le_bytes());
    out.extend_from_slice(s.as_bytes());
}
