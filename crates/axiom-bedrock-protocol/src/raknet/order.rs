use bytes::Bytes;
use std::collections::BTreeMap;
use super::ack::U24;

pub const CHANNELS: usize = 32;

pub struct OrderBuf {
    a: [Channel; CHANNELS],
}

struct Channel {
    a: U24,
    b: BTreeMap<U24, Bytes>,
}

impl Channel {
    fn new() -> Self { Self { a: U24::ZERO, b: BTreeMap::new() } }

    fn insert(&mut self, idx: U24, data: Bytes) -> Vec<Bytes> {
        self.b.insert(idx, data);
        let mut out = Vec::new();
        loop {
            if let Some(d) = self.b.remove(&self.a) {
                out.push(d);
                self.a = self.a.next();
            } else {
                break;
            }
        }
        out
    }
}

impl OrderBuf {
    pub fn new() -> Self {
        Self {
            a: std::array::from_fn(|_| Channel::new()),
        }
    }

    pub fn insert(&mut self, ch: u8, idx: U24, data: Bytes) -> Vec<Bytes> {
        let n = ch as usize % CHANNELS;
        self.a[n].insert(idx, data)
    }

    pub fn next_idx(&self, ch: u8) -> U24 {
        self.a[ch as usize % CHANNELS].a
    }
}

impl Default for OrderBuf {
    fn default() -> Self { Self::new() }
}
