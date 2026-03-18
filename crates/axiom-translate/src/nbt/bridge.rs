use bytes::Bytes;
use crate::error::{Err, R};

pub fn java_nbt_to_bedrock(src: &[u8]) -> R<Vec<u8>> {
    let mut out = Vec::with_capacity(src.len());
    convert_be_to_le(src, &mut out)?;
    Ok(out)
}

pub fn bedrock_nbt_to_java(src: &[u8]) -> R<Vec<u8>> {
    let mut out = Vec::with_capacity(src.len());
    convert_le_to_be(src, &mut out)?;
    Ok(out)
}

fn convert_be_to_le(src: &[u8], out: &mut Vec<u8>) -> R<()> {
    let mut c = src;
    convert_named(&mut c, out, false)?;
    Ok(())
}

fn convert_le_to_be(src: &[u8], out: &mut Vec<u8>) -> R<()> {
    let mut c = src;
    convert_named(&mut c, out, true)?;
    Ok(())
}

fn convert_named(c: &mut &[u8], out: &mut Vec<u8>, le: bool) -> R<()> {
    let tid = need_u8(c)?;
    out.push(tid);
    if tid == 0 { return Ok(()); }
    convert_str(c, out, le)?;
    convert_payload(c, out, tid, le)
}

fn convert_payload(c: &mut &[u8], out: &mut Vec<u8>, tid: u8, le: bool) -> R<()> {
    match tid {
        0  => {}
        1  => { let b = need_u8(c)?; out.push(b); }
        2  => convert_num(c, out, 2, le)?,
        3  => convert_num(c, out, 4, le)?,
        4  => convert_num(c, out, 8, le)?,
        5  => convert_num(c, out, 4, le)?,
        6  => convert_num(c, out, 8, le)?,
        7  => {
            let n = read_i32(c, le)? as usize;
            write_i32(out, n as i32, !le);
            need_n(c, out, n)?;
        }
        8  => convert_str(c, out, le)?,
        9  => {
            let lt = need_u8(c)?; out.push(lt);
            let n  = read_i32(c, le)? as usize;
            write_i32(out, n as i32, !le);
            for _ in 0..n { convert_payload(c, out, lt, le)?; }
        }
        10 => {
            loop {
                let t = need_u8(c)?; out.push(t);
                if t == 0 { break; }
                convert_str(c, out, le)?;
                convert_payload(c, out, t, le)?;
            }
        }
        11 => {
            let n = read_i32(c, le)? as usize;
            write_i32(out, n as i32, !le);
            for _ in 0..n { convert_num(c, out, 4, le)?; }
        }
        12 => {
            let n = read_i32(c, le)? as usize;
            write_i32(out, n as i32, !le);
            for _ in 0..n { convert_num(c, out, 8, le)?; }
        }
        x  => return Err(Err::Nbt(format!("unknown tag {x}"))),
    }
    Ok(())
}

fn convert_num(c: &mut &[u8], out: &mut Vec<u8>, size: usize, src_le: bool) -> R<()> {
    if c.len() < size { return Err(Err::Nbt("eof".into())); }
    let mut bytes = c[..size].to_vec();
    *c = &c[size..];
    bytes.reverse();
    out.extend_from_slice(&bytes);
    Ok(())
}

fn convert_str(c: &mut &[u8], out: &mut Vec<u8>, src_le: bool) -> R<()> {
    if c.len() < 2 { return Err(Err::Nbt("eof in str len".into())); }
    let len = if src_le {
        u16::from_le_bytes([c[0], c[1]]) as usize
    } else {
        u16::from_be_bytes([c[0], c[1]]) as usize
    };
    *c = &c[2..];
    if src_le {
        out.extend_from_slice(&(len as u16).to_be_bytes());
    } else {
        out.extend_from_slice(&(len as u16).to_le_bytes());
    }
    if c.len() < len { return Err(Err::Nbt("eof in str body".into())); }
    out.extend_from_slice(&c[..len]);
    *c = &c[len..];
    Ok(())
}

fn read_i32(c: &mut &[u8], le: bool) -> R<i32> {
    if c.len() < 4 { return Err(Err::Nbt("eof".into())); }
    let v = if le {
        i32::from_le_bytes([c[0],c[1],c[2],c[3]])
    } else {
        i32::from_be_bytes([c[0],c[1],c[2],c[3]])
    };
    *c = &c[4..]; Ok(v)
}

fn write_i32(out: &mut Vec<u8>, v: i32, le: bool) {
    if le { out.extend_from_slice(&v.to_le_bytes()); }
    else  { out.extend_from_slice(&v.to_be_bytes()); }
}

fn need_u8(c: &mut &[u8]) -> R<u8> {
    if c.is_empty() { return Err(Err::Nbt("eof".into())); }
    let v = c[0]; *c = &c[1..]; Ok(v)
}

fn need_n(c: &mut &[u8], out: &mut Vec<u8>, n: usize) -> R<()> {
    if c.len() < n { return Err(Err::Nbt("eof".into())); }
    out.extend_from_slice(&c[..n]);
    *c = &c[n..]; Ok(())
}
