use bytes::{BufMut, BytesMut};
use super::tag::Tag;
use std::collections::HashMap;

pub fn encode(name: &str, tag: &Tag) -> Vec<u8> {
    let mut o = BytesMut::new();
    write_named(&mut o, name, tag);
    o.to_vec()
}

pub fn encode_network(tag: &Tag) -> Vec<u8> {
    let mut o = BytesMut::new();
    o.put_u8(tag.type_id());
    write_payload(&mut o, tag);
    o.to_vec()
}

fn write_named(o: &mut BytesMut, name: &str, tag: &Tag) {
    o.put_u8(tag.type_id());
    write_str(o, name);
    write_payload(o, tag);
}

fn write_payload(o: &mut BytesMut, tag: &Tag) {
    match tag {
        Tag::End             => {}
        Tag::Byte(n)         => o.put_i8(*n),
        Tag::Short(n)        => o.put_i16(*n),
        Tag::Int(n)          => o.put_i32(*n),
        Tag::Long(n)         => o.put_i64(*n),
        Tag::Float(n)        => o.put_f32(*n),
        Tag::Double(n)       => o.put_f64(*n),
        Tag::ByteArray(v)    => {
            o.put_i32(v.len() as i32);
            for b in v { o.put_i8(*b); }
        }
        Tag::String(s)       => write_str(o, s),
        Tag::List(v)         => {
            let tid = v.first().map(|t| t.type_id()).unwrap_or(0);
            o.put_u8(tid);
            o.put_i32(v.len() as i32);
            for t in v { write_payload(o, t); }
        }
        Tag::Compound(m)     => {
            for (k, v) in m { write_named(o, k, v); }
            o.put_u8(0);
        }
        Tag::IntArray(v)     => {
            o.put_i32(v.len() as i32);
            for n in v { o.put_i32(*n); }
        }
        Tag::LongArray(v)    => {
            o.put_i32(v.len() as i32);
            for n in v { o.put_i64(*n); }
        }
    }
}

fn write_str(o: &mut BytesMut, s: &str) {
    o.put_u16(s.len() as u16);
    o.extend_from_slice(s.as_bytes());
}
