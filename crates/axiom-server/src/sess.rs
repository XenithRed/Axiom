use bytes::Bytes;
use std::net::SocketAddr;
use std::sync::Arc;
use tokio::sync::{mpsc, RwLock};
use uuid::Uuid;
use axiom_core::{Agsm, ConnId};

pub type SharedAgsm = Arc<RwLock<Agsm>>;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SessState {
    Handshake,
    Login,
    Config,
    Play,
    Closing,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Edition { Java, Bedrock }

pub struct Sess {
    pub a: ConnId,
    pub b: Edition,
    pub c: SocketAddr,
    pub d: SessState,
    pub e: Option<Uuid>,
    pub f: Option<String>,
    pub g: mpsc::Sender<Bytes>,
    pub h: SharedAgsm,
}

impl Sess {
    pub fn new(
        id:     ConnId,
        ed:     Edition,
        addr:   SocketAddr,
        tx:     mpsc::Sender<Bytes>,
        agsm:   SharedAgsm,
    ) -> Self {
        Self {
            a: id,
            b: ed,
            c: addr,
            d: SessState::Handshake,
            e: None,
            f: None,
            g: tx,
            h: agsm,
        }
    }

    pub async fn send(&self, data: Bytes) -> crate::Result<()> {
        self.g.send(data).await.map_err(|_| crate::Err::Send)
    }

    pub fn set_player(&mut self, uuid: Uuid, name: String) {
        self.e = Some(uuid);
        self.f = Some(name);
    }

    pub fn uuid(&self)  -> Option<Uuid>   { self.e }
    pub fn name(&self)  -> Option<&str>   { self.f.as_deref() }
    pub fn state(&self) -> SessState      { self.d }
    pub fn edition(&self) -> Edition      { self.b }
    pub fn addr(&self)  -> SocketAddr     { self.c }
    pub fn id(&self)    -> ConnId         { self.a }
}

pub struct SessMap {
    a: ahash::AHashMap<ConnId, Sess>,
}

impl SessMap {
    pub fn new() -> Self { Self { a: ahash::AHashMap::new() } }

    pub fn insert(&mut self, s: Sess) { self.a.insert(s.a, s); }

    pub fn remove(&mut self, id: ConnId) -> Option<Sess> { self.a.remove(&id) }

    pub fn get(&self, id: ConnId) -> Option<&Sess> { self.a.get(&id) }

    pub fn get_mut(&mut self, id: ConnId) -> Option<&mut Sess> { self.a.get_mut(&id) }

    pub fn count(&self) -> usize { self.a.len() }

    pub fn iter(&self) -> impl Iterator<Item = &Sess> { self.a.values() }
}
