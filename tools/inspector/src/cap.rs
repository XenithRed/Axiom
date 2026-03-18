use bytes::Bytes;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::net::SocketAddr;
use tokio::net::{TcpListener, TcpStream};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::sync::mpsc;
use std::sync::atomic::{AtomicU64, Ordering};

static SEQ: AtomicU64 = AtomicU64::new(0);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Dir { C2S, S2C }

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Frame {
    pub a: u64,
    pub b: DateTime<Utc>,
    pub c: Dir,
    pub d: Bytes,
    pub e: SocketAddr,
}

impl Frame {
    pub fn new(dir: Dir, data: Bytes, peer: SocketAddr) -> Self {
        Self {
            a: SEQ.fetch_add(1, Ordering::Relaxed),
            b: Utc::now(),
            c: dir,
            d: data,
            e: peer,
        }
    }
}

pub struct Capture {
    pub tx: mpsc::Sender<Frame>,
}

impl Capture {
    pub fn new(tx: mpsc::Sender<Frame>) -> Self {
        Self { tx }
    }

    pub async fn run_proxy(
        self,
        listen: SocketAddr,
        upstream: SocketAddr,
    ) -> crate::Result<()> {
        let ln = TcpListener::bind(listen).await?;
        tracing::info!(%listen, %upstream, "inspector proxy listening");

        loop {
            let (client, peer) = ln.accept().await?;
            let tx = self.tx.clone();
            tokio::spawn(async move {
                if let Err(e) = handle_conn(client, peer, upstream, tx).await {
                    tracing::warn!(%e, "connection error");
                }
            });
        }
    }
}

async fn handle_conn(
    mut client: TcpStream,
    peer: SocketAddr,
    upstream: SocketAddr,
    tx: mpsc::Sender<Frame>,
) -> crate::Result<()> {
    let mut server = TcpStream::connect(upstream).await?;

    let (mut cr, mut cw) = client.split();
    let (mut sr, mut sw) = server.split();

    let tx_c2s = tx.clone();
    let tx_s2c = tx.clone();

    let c2s = async {
        let mut buf = vec![0u8; 65536];
        loop {
            let n = cr.read(&mut buf).await?;
            if n == 0 { break; }
            let data = Bytes::copy_from_slice(&buf[..n]);
            let _ = tx_c2s.send(Frame::new(Dir::C2S, data.clone(), peer)).await;
            sw.write_all(&data).await?;
        }
        Ok::<_, crate::Error>(())
    };

    let s2c = async {
        let mut buf = vec![0u8; 65536];
        loop {
            let n = sr.read(&mut buf).await?;
            if n == 0 { break; }
            let data = Bytes::copy_from_slice(&buf[..n]);
            let _ = tx_s2c.send(Frame::new(Dir::S2C, data.clone(), peer)).await;
            cw.write_all(&data).await?;
        }
        Ok::<_, crate::Error>(())
    };

    tokio::select! {
        r = c2s => r?,
        r = s2c => r?,
    }

    Ok(())
}
