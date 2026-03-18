use clap::Parser;

mod cfg;
mod cli;
mod log;
mod proxy;
mod sess;
mod sig;

#[derive(Debug, thiserror::Error)]
pub enum Err {
    #[error("io: {0}")]
    Io(#[from] std::io::Error),
    #[error("config: {0}")]
    Config(String),
    #[error("upstream: {0}")]
    Upstream(String),
    #[error("send failed")]
    Send,
    #[error("session: {0}")]
    Sess(String),
}

pub type Result<T> = std::result::Result<T, Err>;

#[tokio::main]
async fn main() -> Result<()> {
    let cli = cli::Cli::parse();

    let mut config = cfg::Cfg::load(&cli.config);
    config.apply_overrides(&cli);

    log::init(&config.d.a, config.d.b);

    print_banner();

    tracing::info!(
        version = env!("CARGO_PKG_VERSION"),
        bedrock = %config.bedrock_addr(),
        java    = "{}:{}",
        config.java_host(),
        config.java_port(),
        "axiom starting"
    );

    if cli.dry_run {
        tracing::info!("dry-run mode — config ok, exiting");
        return Ok(());
    }

    let shutdown = sig::Shutdown::new();
    let rx = shutdown.subscribe();

    let proxy = proxy::Proxy::new(config);

    tokio::select! {
        r = proxy.run(rx) => {
            if let Err(e) = r {
                tracing::error!(%e, "proxy error");
                std::process::exit(1);
            }
        }
        _ = shutdown.wait_for_signal() => {}
    }

    tracing::info!("axiom stopped");
    Ok(())
}

fn print_banner() {
    println!();
    println!("  ▄▀█ ▀▄▀ █ █▀█ █▀▄▀█");
    println!("  █▀█ █░█ █ █▄█ █░▀░█");
    println!();
    println!("  Universal Minecraft Protocol Bridge");
    println!("  The game is the same. The wire never was.");
    println!();
}
