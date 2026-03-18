use bytes::{Buf, BufMut, Bytes, BytesMut};

pub struct RingBuf {
    a: BytesMut,
    b: usize,
}

impl RingBuf {
    pub fn new(cap: usize) -> Self {
        Self { a: BytesMut::with_capacity(cap), b: 0 }
    }

    pub fn write(&mut self, data: &[u8]) {
        self.a.extend_from_slice(data);
    }

    pub fn peek(&self, n: usize) -> Option<&[u8]> {
        if self.a.len() >= n { Some(&self.a[..n]) } else { None }
    }

    pub fn consume(&mut self, n: usize) -> Bytes {
        let s = self.a.split_to(n.min(self.a.len()));
        self.b += s.len();
        s.freeze()
    }

    pub fn available(&self) -> usize { self.a.len() }
    pub fn is_empty(&self) -> bool   { self.a.is_empty() }
    pub fn total_read(&self) -> usize { self.b }

    pub fn try_read_frame(&mut self) -> Option<Bytes> {
        let (n, hdr) = read_var32_prefix(&self.a)?;
        let total = hdr + n;
        if self.a.len() < total { return None; }
        self.a.advance(n);
        Some(self.a.split_to(hdr).freeze())
    }
}

fn read_var32_prefix(b: &BytesMut) -> Option<(usize, usize)> {
    let (mut val, mut s, mut i) = (0u32, 0u32, 0usize);
    loop {
        if i >= b.len() || i >= 5 { return None; }
        let x = b[i]; i += 1;
        val |= ((x & 0x7F) as u32) << s;
        if x & 0x80 == 0 { return Some((i, val as usize)); }
        s += 7;
    }
}
