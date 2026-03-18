use bytes::Bytes;
use std::net::SocketAddr;
use std::sync::Arc;
use tokio::net::{TcpListener, TcpStream, UdpSocket};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::sync::{mpsc, RwLock};
use axiom_core::{Agsm, ConnId};
use crate::{cfg::Cfg, sess::{Edition, Sess, SessMap, SessState, SharedAgsm}, Err, Result};

const CHAN_CAP: usize = 512;
const BUF: usize = 65536;

pub struct Proxy {
    pub a: Arc<Cfg>,
    pub b: SharedAgsm,
}

impl Proxy {
    pub fn new(cfg: Cfg) -> Self {
        Self {
            a: Arc::new(cfg),
            b: Arc::new(RwLock::new(Agsm::new())),
        }
    }

    pub async fn run(self, mut shutdown: tokio::sync::watch::Receiver<bool>) -> Result<()> {
        let bedrock_addr: SocketAddr = self.a.bedrock_addr().parse()
            .map_err(|e| Err::Config(format!("invalid bedrock bind: {e}")))?;

        let java_addr = format!("{}:{}", self.a.java_host(), self.a.java_port());

        tracing::info!(
            bedrock = %bedrock_addr,
            java    = %java_addr,
            motd    = %self.a.motd(),
            "axiom proxy starting"
        );

        let java_listener  = TcpListener::bind(format!("0.0.0.0:{}", self.a.java_port() + 10000)).await?;
        let bedrock_socket = Arc::new(UdpSocket::bind(bedrock_addr).await?);

        tracing::info!("listeners bound — bridge active");

        let cfg_java = self.a.clone();
        let agsm_java = self.b.clone();
        let mut shutdown_java = shutdown.clone();

        let java_task = tokio::spawn(async move {
            loop {
                tokio::select! {
                    Ok((stream, peer)) = java_listener.accept() => {
                        let cfg  = cfg_java.clone();
                        let agsm = agsm_java.clone();
                        tokio::spawn(async move {
                            if let Err(e) = handle_java(stream, peer, cfg, agsm).await {
                                tracing::warn!(%peer, %e, "java session error");
                            }
                        });
                    }
                    _ = shutdown_java.changed() => {
                        if *shutdown_java.borrow() { break; }
                    }
                }
            }
        });

        let cfg_bedrock  = self.a.clone();
        let agsm_bedrock = self.b.clone();
        let sock_b       = bedrock_socket.clone();
        let java_upstream = java_addr.clone();
        let mut shutdown_bedrock = shutdown.clone();

        let bedrock_task = tokio::spawn(async move {
            let mut buf = vec![0u8; BUF];
            loop {
                tokio::select! {
                    Ok((n, peer)) = sock_b.recv_from(&mut buf) => {
                        let data = Bytes::copy_from_slice(&buf[..n]);
                        let sock = sock_b.clone();
                        let cfg  = cfg_bedrock.clone();
                        let agsm = agsm_bedrock.clone();
                        let up   = java_upstream.clone();
                        tokio::spawn(async move {
                            if let Err(e) = handle_bedrock(data, peer, sock, cfg, agsm, up).await {
                                tracing::warn!(%peer, %e, "bedrock packet error");
                            }
                        });
                    }
                    _ = shutdown_bedrock.changed() => {
                        if *shutdown_bedrock.borrow() { break; }
                    }
                }
            }
        });

        shutdown.changed().await.ok();
        tracing::info!("shutdown signal received — stopping");

        java_task.abort();
        bedrock_task.abort();

        Ok(())
    }
}

async fn handle_java(
    mut stream: TcpStream,
    peer:       SocketAddr,
    cfg:        Arc<Cfg>,
    agsm:       SharedAgsm,
) -> Result<()> {
    let id = ConnId::next();
    let (tx, mut rx) = mpsc::channel::<Bytes>(CHAN_CAP);

    let sess = Sess::new(id, Edition::Java, peer, tx, agsm.clone());

    tracing::debug!(%id, %peer, "java client connected");

    let java_target = format!("{}:{}", cfg.java_host(), cfg.java_port());
    let mut upstream = TcpStream::connect(&java_target).await
        .map_err(|e| Err::Upstream(e.to_string()))?;

    let (mut cr, mut cw) = stream.split();
    let (mut ur, mut uw) = upstream.split();

    let agsm_c2s = agsm.clone();
    let id_c2s   = id;

    let c2s = async {
        let mut buf = vec![0u8; BUF];
        loop {
            let n = cr.read(&mut buf).await?;
            if n == 0 { break; }
            let raw = Bytes::copy_from_slice(&buf[..n]);
            uw.write_all(&raw).await?;
        }
        Ok::<_, std::io::Error>(())
    };

    let s2c = async {
        let mut buf = vec![0u8; BUF];
        loop {
            let n = ur.read(&mut buf).await?;
            if n == 0 { break; }
            let raw = Bytes::copy_from_slice(&buf[..n]);
            cw.write_all(&raw).await?;
        }
        Ok::<_, std::io::Error>(())
    };

    tokio::select! {
        r = c2s => { r.map_err(Err::Io)?; }
        r = s2c => { r.map_err(Err::Io)?; }
    }

    tracing::debug!(%id, %peer, "java client disconnected");
    Ok(())
}

async fn handle_bedrock(
    data:     Bytes,
    peer:     SocketAddr,
    sock:     Arc<UdpSocket>,
    cfg:      Arc<Cfg>,
    agsm:     SharedAgsm,
    upstream: String,
) -> Result<()> {
    tracing::trace!(%peer, bytes = data.len(), "bedrock datagram");
    Ok(())
}
