use bytes::{Buf, BufMut, Bytes, BytesMut};
use crate::error::{Err, R};

pub const ACK_ID: u8 = 0xC0;
pub const NAK_ID: u8 = 0xA0;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct U24(u32);

impl U24 {
    pub const ZERO: Self = Self(0);
    pub const MAX:  Self = Self(0x00FF_FFFF);

    #[inline] pub fn new(n: u32) -> Self { Self(n & 0x00FF_FFFF) }
    #[inline] pub fn get(self) -> u32 { self.0 }
    #[inline] pub fn next(self) -> Self { Self((self.0 + 1) & 0x00FF_FFFF) }

    pub fn diff(self, rhs: Self) -> i32 {
        let a = self.0 as i32;
        let b = rhs.0 as i32;
        let d = a - b;
        if      d >  0x7F_FFFF { d - 0x100_0000 }
        else if d < -0x7F_FFFF { d + 0x100_0000 }
        else                   { d }
    }

    #[inline]
    pub fn read(b: &mut &[u8]) -> R<Self> {
        if b.len() < 3 { return Err(Err::Eof); }
        let n = b[0] as u32 | (b[1] as u32) << 8 | (b[2] as u32) << 16;
        *b = &b[3..];
        Ok(Self(n))
    }

    #[inline]
    pub fn write(self, out: &mut BytesMut) {
        let n = self.0;
        out.put_u8((n & 0xFF) as u8);
        out.put_u8(((n >> 8)  & 0xFF) as u8);
        out.put_u8(((n >> 16) & 0xFF) as u8);
    }
}

#[derive(Debug, Clone)]
pub struct AckRecord {
    pub a: U24,
    pub b: U24,
}

impl AckRecord {
    pub fn single(n: U24) -> Self { Self { a: n, b: n } }
    pub fn range(lo: U24, hi: U24) -> Self { Self { a: lo, b: hi } }
    pub fn is_single(&self) -> bool { self.a == self.b }
}

#[derive(Debug, Clone)]
pub struct AckPkt {
    pub a: u8,
    pub b: Vec<AckRecord>,
}

impl AckPkt {
    pub fn ack(records: Vec<AckRecord>) -> Self { Self { a: ACK_ID, b: records } }
    pub fn nak(records: Vec<AckRecord>) -> Self { Self { a: NAK_ID, b: records } }

    pub fn encode(&self) -> Bytes {
        let mut out = BytesMut::new();
        out.put_u8(self.a);
        out.put_u16(self.b.len() as u16);
        for r in &self.b {
            if r.is_single() {
                out.put_u8(1);
                r.a.write(&mut out);
            } else {
                out.put_u8(0);
                r.a.write(&mut out);
                r.b.write(&mut out);
            }
        }
        out.freeze()
    }

    pub fn decode(raw: &[u8]) -> R<Self> {
        if raw.len() < 4 { return Err(Err::Eof); }
        let id = raw[0];
        if id != ACK_ID && id != NAK_ID { return Err(Err::BadId(id)); }
        let mut cur = &raw[1..];
        if cur.len() < 2 { return Err(Err::Eof); }
        let count = cur.get_u16() as usize;
        let mut records = Vec::with_capacity(count);
        for _ in 0..count {
            if cur.is_empty() { return Err(Err::Eof); }
            let single = cur.get_u8() == 1;
            if single {
                let n = U24::read(&mut cur)?;
                records.push(AckRecord::single(n));
            } else {
                let lo = U24::read(&mut cur)?;
                let hi = U24::read(&mut cur)?;
                records.push(AckRecord::range(lo, hi));
            }
        }
        Ok(Self { a: id, b: records })
    }

    pub fn seq_iter(&self) -> impl Iterator<Item = U24> + '_ {
        self.b.iter().flat_map(|r| {
            let lo = r.a.get();
            let hi = r.b.get();
            (lo..=hi).map(U24::new)
        })
    }
}

pub fn encode_acks(seqs: &[U24], id: u8) -> Bytes {
    let records = compress_ranges(seqs);
    AckPkt { a: id, b: records }.encode()
}

fn compress_ranges(seqs: &[U24]) -> Vec<AckRecord> {
    if seqs.is_empty() { return vec![]; }
    let mut s: Vec<U24> = seqs.to_vec();
    s.sort_unstable();
    s.dedup();
    let mut out = Vec::new();
    let mut lo = s[0];
    let mut hi = s[0];
    for &n in &s[1..] {
        if n.get() == hi.get() + 1 {
            hi = n;
        } else {
            out.push(AckRecord::range(lo, hi));
            lo = n;
            hi = n;
        }
    }
    out.push(AckRecord::range(lo, hi));
    out
}
