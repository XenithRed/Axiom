use std::collections::BTreeSet;
use super::ack::U24;

const INIT: u32 = 64;
const MIN:  u32 = 16;
const MAX:  u32 = 2048;

pub struct Window {
    a: u32,
    b: u32,
    c: BTreeSet<U24>,
    d: U24,
    e: U24,
}

impl Window {
    pub fn new() -> Self {
        Self {
            a: INIT,
            b: 0,
            c: BTreeSet::new(),
            d: U24::ZERO,
            e: U24::ZERO,
        }
    }

    pub fn can_send(&self) -> bool {
        self.b < self.a
    }

    pub fn on_send(&mut self, seq: U24) {
        self.c.insert(seq);
        self.b += 1;
    }

    pub fn on_ack(&mut self, seq: U24) {
        if self.c.remove(&seq) {
            self.b = self.b.saturating_sub(1);
            if self.a < MAX {
                self.a = (self.a + 1).min(MAX);
            }
        }
    }

    pub fn on_nak(&mut self) {
        self.a = (self.a / 2).max(MIN);
    }

    pub fn next_send_seq(&mut self) -> U24 {
        let n = self.d;
        self.d = self.d.next();
        n
    }

    pub fn next_recv_seq(&mut self) -> U24 {
        let n = self.e;
        self.e = self.e.next();
        n
    }

    pub fn unacked(&self) -> &BTreeSet<U24> {
        &self.c
    }

    pub fn size(&self) -> u32 { self.a }
}

impl Default for Window {
    fn default() -> Self { Self::new() }
}
