use bytes::Bytes;
use tokio::sync::mpsc;
use std::net::SocketAddr;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use super::pkt::{Dir, Edition, Envelope};
use crate::error::{Err, R};

static CONN_ID: std::sync::atomic::AtomicU64 = std::sync::atomic::AtomicU64::new(1);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ConnId(pub u64);

impl ConnId {
    pub fn next() -> Self {
        Self(CONN_ID.fetch_add(1, Ordering::Relaxed))
    }
}

impl std::fmt::Display for ConnId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "C{}", self.0)
    }
}

pub struct Conn {
    pub a: ConnId,
    pub b: Edition,
    pub c: SocketAddr,
    pub d: mpsc::Sender<Envelope>,
    pub e: mpsc::Receiver<Envelope>,
    pub f: Arc<AtomicBool>,
}

impl Conn {
    pub fn pair(edition: Edition, addr: SocketAddr, cap: usize) -> (Self, Self) {
        let id  = ConnId::next();
        let closed = Arc::new(AtomicBool::new(false));

        let (tx_a, rx_b) = mpsc::channel(cap);
        let (tx_b, rx_a) = mpsc::channel(cap);

        let a = Self { a: id, b: edition, c: addr, d: tx_a, e: rx_a, f: closed.clone() };
        let b = Self { a: id, b: edition, c: addr, d: tx_b, e: rx_b, f: closed };

        (a, b)
    }

    pub async fn send(&self, id: i32, data: Bytes, dir: Dir) -> R<()> {
        if self.f.load(Ordering::Relaxed) { return Err(Err::Closed); }
        let env = Envelope::new(self.b, dir, id, data);
        self.d.send(env).await.map_err(|_| Err::Send)
    }

    pub async fn recv(&mut self) -> R<Envelope> {
        self.e.recv().await.ok_or(Err::Closed)
    }

    pub fn try_recv(&mut self) -> Option<Envelope> {
        self.e.try_recv().ok()
    }

    pub fn close(&self) {
        self.f.store(true, Ordering::Relaxed);
    }

    pub fn is_closed(&self) -> bool {
        self.f.load(Ordering::Relaxed)
    }

    pub fn id(&self)      -> ConnId     { self.a }
    pub fn edition(&self) -> Edition    { self.b }
    pub fn addr(&self)    -> SocketAddr { self.c }
}
