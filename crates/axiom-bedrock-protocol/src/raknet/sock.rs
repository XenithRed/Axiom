use bytes::Bytes;
use std::net::SocketAddr;
use std::sync::Arc;
use tokio::net::UdpSocket;
use tokio::sync::{mpsc, Mutex};
use tokio::time::{interval, Duration};
use crate::error::{Err, R};
use super::{
    ack::{AckPkt, ACK_ID, NAK_ID},
    arq::{ArqRecv, ArqSend},
    datagram::Datagram,
};

const TICK_MS:   u64 = 10;
const BUF_SZ:    usize = 2048;
const CHAN_CAP:  usize = 512;

pub struct RakSock {
    a: Arc<UdpSocket>,
    b: SocketAddr,
    c: Arc<Mutex<ArqSend>>,
    d: Arc<Mutex<ArqRecv>>,
    e: mpsc::Sender<Bytes>,
    f: mpsc::Receiver<Bytes>,
}

impl RakSock {
    pub async fn connect(local: &str, remote: SocketAddr) -> R<Self> {
        let sock = UdpSocket::bind(local).await?;
        sock.connect(remote).await?;
        let sock = Arc::new(sock);
        let send = Arc::new(Mutex::new(ArqSend::new()));
        let recv = Arc::new(Mutex::new(ArqRecv::new()));
        let (tx, rx) = mpsc::channel(CHAN_CAP);

        let s2 = sock.clone();
        let sd2 = send.clone();
        let rv2 = recv.clone();
        let tx2 = tx.clone();

        tokio::spawn(async move {
            io_loop(s2, sd2, rv2, tx2).await;
        });

        Ok(Self { a: sock, b: remote, c: send, d: recv, e: tx, f: rx })
    }

    pub async fn send(&self, data: Bytes) -> R<()> {
        let mut sd = self.c.lock().await;
        let pkts = sd.send_reliable_ord(data, 0);
        drop(sd);
        for p in pkts {
            self.a.send(&p).await?;
        }
        Ok(())
    }

    pub async fn recv(&mut self) -> R<Bytes> {
        self.f.recv().await.ok_or(Err::Closed)
    }
}

async fn io_loop(
    sock:  Arc<UdpSocket>,
    send:  Arc<Mutex<ArqSend>>,
    recv:  Arc<Mutex<ArqRecv>>,
    tx:    mpsc::Sender<Bytes>,
) {
    let mut buf   = vec![0u8; BUF_SZ];
    let mut tick  = interval(Duration::from_millis(TICK_MS));

    loop {
        tokio::select! {
            n = sock.recv(&mut buf) => {
                let n = match n { Ok(n) => n, Err(_) => break };
                let raw = &buf[..n];
                let id  = raw[0];

                if id == ACK_ID {
                    if let Ok(pkt) = AckPkt::decode(raw) {
                        let mut sd = send.lock().await;
                        for seq in pkt.seq_iter() { sd.on_ack(seq); }
                    }
                } else if id == NAK_ID {
                    if let Ok(pkt) = AckPkt::decode(raw) {
                        let mut sd = send.lock().await;
                        let mut out = Vec::new();
                        for seq in pkt.seq_iter() {
                            if let Some(b) = sd.on_nak(seq) { out.push(b); }
                        }
                        drop(sd);
                        for p in out { let _ = sock.send(&p).await; }
                    }
                } else if Datagram::is_data(id) {
                    let mut rv = recv.lock().await;
                    if let Ok(pkts) = rv.on_datagram(raw) {
                        drop(rv);
                        for p in pkts { let _ = tx.send(p).await; }
                    }
                    let mut rv = recv.lock().await;
                    if let Some(ack) = rv.drain_acks() {
                        drop(rv);
                        let _ = sock.send(&ack).await;
                    }
                }
            }

            _ = tick.tick() => {
                {
                    let mut rv = recv.lock().await;
                    if let Some(nak) = rv.drain_naks() {
                        drop(rv);
                        let _ = sock.send(&nak).await;
                    }
                }
                {
                    let mut sd = send.lock().await;
                    let pkts = sd.retransmit_expired();
                    drop(sd);
                    for p in pkts { let _ = sock.send(&p).await; }
                }
            }
        }
    }
}
