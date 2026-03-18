use clap::Parser;
use tokio::sync::mpsc;
use std::net::SocketAddr;
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::Arc;

mod cap;
mod filter;
mod fmt;
mod parse;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("io: {0}")] Io(#[from] std::io::Error),
    #[error("addr: {0}")] Addr(#[from] std::net::AddrParseError),
}
pub type Result<T> = std::result::Result<T, Error>;

#[derive(Parser, Debug)]
#[command(name = "inspector", about = "Axiom packet capture and analysis tool")]
pub struct Args {
    #[arg(short, long, default_value = "127.0.0.1:25564")]
    pub listen: String,

    #[arg(short, long, default_value = "127.0.0.1:25565")]
    pub upstream: String,

    #[arg(short, long, help = "Filter by direction: c2s or s2c")]
    pub direction: Option<String>,

    #[arg(short = 'i', long, help = "Filter by packet ID (decimal)")]
    pub packet_id: Option<i32>,

    #[arg(short, long, help = "Filter by packet name substring")]
    pub name: Option<String>,

    #[arg(long, help = "Minimum packet size in bytes")]
    pub min_bytes: Option<usize>,

    #[arg(long, help = "Maximum packet size in bytes")]
    pub max_bytes: Option<usize>,

    #[arg(short, long, help = "Verbose: show decoded fields")]
    pub verbose: bool,

    #[arg(short = 'x', long, help = "Show hex dump (requires --verbose)")]
    pub hex: bool,

    #[arg(long, default_value = "true", help = "Disable color output")]
    pub color: bool,

    #[arg(short, long, help = "Write captured frames to JSON file")]
    pub output: Option<String>,

    #[arg(long, default_value = "4096", help = "Channel buffer size")]
    pub buf: usize,
}

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::from_default_env()
                .add_directive("inspector=info".parse().unwrap()),
        )
        .init();

    let args = Args::parse();
    let listen:   SocketAddr = args.listen.parse()?;
    let upstream: SocketAddr = args.upstream.parse()?;

    let filt = filter::parse_filter(&args);
    let fmtr = fmt::Fmt::new(args.color, args.verbose, args.hex);

    let (tx, mut rx) = mpsc::channel::<cap::Frame>(args.buf);

    let total_pkts  = Arc::new(AtomicU64::new(0));
    let total_c2s   = Arc::new(AtomicU64::new(0));
    let total_s2c   = Arc::new(AtomicU64::new(0));
    let total_bytes = Arc::new(AtomicU64::new(0));

    let tp = total_pkts.clone();
    let tc = total_c2s.clone();
    let ts = total_s2c.clone();
    let tb = total_bytes.clone();

    let output_path = args.output.clone();

    let consumer = tokio::spawn(async move {
        let mut writer: Option<std::fs::File> = output_path.as_deref().map(|p| {
            std::fs::File::create(p).expect("cannot open output file")
        });

        while let Some(frame) = rx.recv().await {
            let parsed = parse::parse(&frame);

            tp.fetch_add(1, Ordering::Relaxed);
            tb.fetch_add(frame.d.len() as u64, Ordering::Relaxed);
            match frame.c {
                cap::Dir::C2S => { tc.fetch_add(1, Ordering::Relaxed); }
                cap::Dir::S2C => { ts.fetch_add(1, Ordering::Relaxed); }
            }

            if filt.matches(&parsed) {
                fmtr.print(&parsed, &frame.d);
            }

            if let Some(ref mut w) = writer {
                use std::io::Write;
                if let Ok(json) = serde_json::to_string(&serde_json::json!({
                    "seq":   frame.a,
                    "ts":    frame.b.to_rfc3339(),
                    "dir":   format!("{:?}", frame.c),
                    "id":    parsed.c,
                    "name":  parsed.d,
                    "bytes": frame.d.len(),
                    "hex":   hex::encode(&frame.d),
                })) {
                    let _ = writeln!(w, "{json}");
                }
            }
        }
    });

    let capture = cap::Capture::new(tx);
    capture.run_proxy(listen, upstream).await?;

    consumer.await.ok();

    let tp = total_pkts.load(Ordering::Relaxed);
    let tc = total_c2s.load(Ordering::Relaxed);
    let ts = total_s2c.load(Ordering::Relaxed);
    let tb = total_bytes.load(Ordering::Relaxed);
    fmt::Fmt::new(args.color, true, false).print_stats(tp, tc, ts, tb);

    Ok(())
}
