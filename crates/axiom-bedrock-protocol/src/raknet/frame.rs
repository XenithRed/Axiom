use bytes::{Buf, BufMut, Bytes, BytesMut};
use crate::error::{Err, R};
use super::ack::U24;

pub const FRAG_BIT: u8 = 0x10;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum Rel {
    Unreliable          = 0,
    UnreliableSeq       = 1,
    Reliable            = 2,
    ReliableOrd         = 3,
    ReliableSeq         = 4,
    UnreliableAck       = 5,
    ReliableAck         = 6,
    ReliableOrdAck      = 7,
}

impl Rel {
    pub fn from_u8(n: u8) -> R<Self> {
        match n {
            0 => Ok(Self::Unreliable),
            1 => Ok(Self::UnreliableSeq),
            2 => Ok(Self::Reliable),
            3 => Ok(Self::ReliableOrd),
            4 => Ok(Self::ReliableSeq),
            5 => Ok(Self::UnreliableAck),
            6 => Ok(Self::ReliableAck),
            7 => Ok(Self::ReliableOrdAck),
            x => Err(Err::BadRel(x)),
        }
    }

    #[inline] pub fn is_reliable(self) -> bool  { (self as u8) >= 2 }
    #[inline] pub fn is_sequenced(self) -> bool { matches!(self, Self::UnreliableSeq | Self::ReliableSeq) }
    #[inline] pub fn is_ordered(self) -> bool   { matches!(self, Self::ReliableOrd | Self::ReliableOrdAck) }
}

#[derive(Debug, Clone)]
pub struct FragInfo {
    pub a: u32,
    pub b: u16,
    pub c: u32,
}

#[derive(Debug, Clone)]
pub struct Frame {
    pub a: Rel,
    pub b: Option<U24>,
    pub c: Option<U24>,
    pub d: Option<(U24, u8)>,
    pub e: Option<FragInfo>,
    pub f: Bytes,
}

impl Frame {
    pub fn unreliable(data: Bytes) -> Self {
        Self { a: Rel::Unreliable, b: None, c: None, d: None, e: None, f: data }
    }

    pub fn reliable(seq: U24, data: Bytes) -> Self {
        Self { a: Rel::Reliable, b: Some(seq), c: None, d: None, e: None, f: data }
    }

    pub fn reliable_ord(seq: U24, ord: U24, ch: u8, data: Bytes) -> Self {
        Self { a: Rel::ReliableOrd, b: Some(seq), c: None, d: Some((ord, ch)), e: None, f: data }
    }

    pub fn encode(&self, out: &mut BytesMut) {
        let flags = ((self.a as u8) << 5) | if self.e.is_some() { FRAG_BIT } else { 0 };
        out.put_u8(flags);
        let bit_len = (self.f.len() as u16) * 8;
        out.put_u16(bit_len);
        if let Some(rsi) = self.b {
            rsi.write(out);
        }
        if let Some(si) = self.c {
            si.write(out);
        }
        if let Some((oi, ch)) = self.d {
            oi.write(out);
            out.put_u8(ch);
        }
        if let Some(ref fi) = self.e {
            out.put_u32(fi.a);
            out.put_u16(fi.b);
            out.put_u32(fi.c);
        }
        out.extend_from_slice(&self.f);
    }

    pub fn decode(cur: &mut &[u8]) -> R<Self> {
        if cur.len() < 3 { return Err(Err::Eof); }
        let flags  = cur.get_u8();
        let rel    = Rel::from_u8(flags >> 5)?;
        let is_frag = flags & FRAG_BIT != 0;
        let bits   = cur.get_u16();
        let blen   = ((bits + 7) / 8) as usize;

        let rsi = if rel.is_reliable() { Some(U24::read(cur)?) } else { None };
        let si  = if rel.is_sequenced()  { Some(U24::read(cur)?) } else { None };
        let d   = if rel.is_ordered() {
            let oi = U24::read(cur)?;
            if cur.is_empty() { return Err(Err::Eof); }
            let ch = cur.get_u8();
            Some((oi, ch))
        } else { None };

        let fi = if is_frag {
            if cur.len() < 10 { return Err(Err::Eof); }
            let csz = cur.get_u32();
            let cid = cur.get_u16();
            let idx = cur.get_u32();
            Some(FragInfo { a: csz, b: cid, c: idx })
        } else { None };

        if cur.len() < blen { return Err(Err::Eof); }
        let data = Bytes::copy_from_slice(&cur[..blen]);
        *cur = &cur[blen..];

        Ok(Self { a: rel, b: rsi, c: si, d, e: fi, f: data })
    }

    pub fn encoded_len(&self) -> usize {
        let mut n = 3;
        if self.b.is_some() { n += 3; }
        if self.c.is_some() { n += 3; }
        if self.d.is_some() { n += 4; }
        if self.e.is_some() { n += 10; }
        n += self.f.len();
        n
    }
}
