use bytes::{BufMut, Bytes, BytesMut};
use std::collections::HashMap;
use crate::error::{Err, R};

const MAX_FRAGS: u32 = 1024;
const MAX_COMPOUND: usize = 256;

struct Entry {
    a: Vec<Option<Bytes>>,
    b: u32,
    c: u32,
}

pub struct FragBuf {
    a: HashMap<u16, Entry>,
}

impl FragBuf {
    pub fn new() -> Self { Self { a: HashMap::new() } }

    pub fn insert(&mut self, id: u16, idx: u32, total: u32, data: Bytes) -> R<Option<Bytes>> {
        if total == 0 || total > MAX_FRAGS { return Err(Err::FragOverflow(id, idx, total)); }
        if idx >= total { return Err(Err::FragOverflow(id, idx, total)); }
        if self.a.len() >= MAX_COMPOUND && !self.a.contains_key(&id) {
            return Err(Err::TooLarge(self.a.len()));
        }
        let e = self.a.entry(id).or_insert_with(|| Entry {
            a: vec![None; total as usize],
            b: 0,
            c: total,
        });
        if e.a[idx as usize].is_none() {
            e.a[idx as usize] = Some(data);
            e.b += 1;
        }
        if e.b == e.c {
            let n = e.c as usize;
            let mut out = BytesMut::new();
            let parts: Vec<Bytes> = e.a.iter_mut().map(|x| x.take().unwrap()).collect();
            for p in parts { out.put_slice(&p); }
            self.a.remove(&id);
            return Ok(Some(out.freeze()));
        }
        Ok(None)
    }

    pub fn prune(&mut self, id: u16) {
        self.a.remove(&id);
    }
}

impl Default for FragBuf {
    fn default() -> Self { Self::new() }
}
