use bytes::Bytes;
use std::collections::{BTreeMap, VecDeque};
use std::time::Instant;
use super::{
    ack::{encode_acks, AckPkt, U24, ACK_ID, NAK_ID},
    datagram::{Datagram, MTU},
    fragment::{FragBuf},
    frame::{Frame, FragInfo, Rel},
    order::OrderBuf,
    rtt::Rtt,
    window::Window,
};
use crate::error::{Err, R};

pub struct ArqSend {
    a: Window,
    b: Rtt,
    c: u32,
    d: U24,
    e: U24,
    f: BTreeMap<U24, (Datagram, Instant)>,
    g: VecDeque<Datagram>,
    h: u16,
}

pub struct ArqRecv {
    a: FragBuf,
    b: OrderBuf,
    c: Vec<U24>,
    d: Vec<U24>,
    e: U24,
}

impl ArqSend {
    pub fn new() -> Self {
        Self {
            a: Window::new(),
            b: Rtt::new(),
            c: 0,
            d: U24::ZERO,
            e: U24::ZERO,
            f: BTreeMap::new(),
            g: VecDeque::new(),
            h: 0,
        }
    }

    pub fn send_reliable_ord(&mut self, data: Bytes, ch: u8) -> Vec<Bytes> {
        let chunks = split_mtu(data);
        let total  = chunks.len() as u32;
        let cid    = self.next_compound_id();
        let mut out = Vec::new();
        for (idx, chunk) in chunks.into_iter().enumerate() {
            let rsi = self.d;
            self.d  = self.d.next();
            let oi  = self.e;
            self.e  = self.e.next();
            let fi = if total > 1 {
                Some(FragInfo { a: total, b: cid, c: idx as u32 })
            } else { None };
            let mut f = Frame::reliable_ord(rsi, oi, ch, chunk);
            f.e = fi;
            let mut dg = Datagram::new(self.a.next_send_seq());
            dg.push(f);
            self.a.on_send(dg.a);
            self.f.insert(dg.a, (dg.clone(), Instant::now()));
            out.push(dg.encode());
        }
        out
    }

    pub fn on_ack(&mut self, seq: U24) {
        if let Some((_, t)) = self.f.remove(&seq) {
            let elapsed = t.elapsed();
            self.b.update(elapsed);
            self.a.on_ack(seq);
        }
    }

    pub fn on_nak(&mut self, seq: U24) -> Option<Bytes> {
        self.a.on_nak();
        self.f.get(&seq).map(|(dg, _)| dg.encode())
    }

    pub fn retransmit_expired(&mut self) -> Vec<Bytes> {
        let rto = self.b.rto();
        let now = Instant::now();
        let mut out = Vec::new();
        for (_, (dg, sent)) in &mut self.f {
            if now.duration_since(*sent) >= rto {
                *sent = now;
                out.push(dg.encode());
            }
        }
        out
    }

    fn next_compound_id(&mut self) -> u16 {
        let n = self.h;
        self.h = self.h.wrapping_add(1);
        n
    }
}

impl ArqRecv {
    pub fn new() -> Self {
        Self {
            a: FragBuf::new(),
            b: OrderBuf::new(),
            c: Vec::new(),
            d: Vec::new(),
            e: U24::ZERO,
        }
    }

    pub fn on_datagram(&mut self, raw: &[u8]) -> R<Vec<Bytes>> {
        let dg = Datagram::decode(raw)?;
        self.c.push(dg.a);
        let exp = self.e;
        if dg.a.diff(exp) > 0 {
            for n in exp.get()..dg.a.get() {
                self.d.push(U24::new(n));
            }
        }
        self.e = U24::new(dg.a.get().max(self.e.get()) + 1);

        let mut out = Vec::new();
        for f in dg.b {
            let assembled = if let Some(ref fi) = f.e {
                self.a.insert(fi.b, fi.c, fi.a, f.f.clone())?
            } else {
                Some(f.f.clone())
            };
            if let Some(data) = assembled {
                if let Some((oi, ch)) = f.d {
                    let ready = self.b.insert(ch, oi, data);
                    out.extend(ready);
                } else {
                    out.push(data);
                }
            }
        }
        Ok(out)
    }

    pub fn drain_acks(&mut self) -> Option<Bytes> {
        if self.c.is_empty() { return None; }
        let seqs: Vec<U24> = self.c.drain(..).collect();
        Some(encode_acks(&seqs, ACK_ID))
    }

    pub fn drain_naks(&mut self) -> Option<Bytes> {
        if self.d.is_empty() { return None; }
        let seqs: Vec<U24> = self.d.drain(..).collect();
        Some(encode_acks(&seqs, NAK_ID))
    }
}

fn split_mtu(data: Bytes) -> Vec<Bytes> {
    let overhead = 3 + 3 + 4 + 3 + 10;
    let chunk_sz = MTU.saturating_sub(overhead).max(64);
    if data.len() <= chunk_sz {
        return vec![data];
    }
    let mut out = Vec::new();
    let mut off = 0;
    while off < data.len() {
        let end = (off + chunk_sz).min(data.len());
        out.push(data.slice(off..end));
        off = end;
    }
    out
}

impl Default for ArqSend { fn default() -> Self { Self::new() } }
impl Default for ArqRecv { fn default() -> Self { Self::new() } }
