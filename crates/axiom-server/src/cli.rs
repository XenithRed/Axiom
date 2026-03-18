use clap::Parser;
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[command(
    name    = "axiom",
    version = env!("CARGO_PKG_VERSION"),
    about   = "Axiom — Universal Minecraft Protocol Bridge",
    long_about = "
Axiom bridges Minecraft Bedrock Edition clients to Java Edition servers
and vice versa, with zero modification required on either side.

The game is the same. The wire never was.
"
)]
pub struct Cli {
    #[arg(
        short, long,
        default_value = "config/default.toml",
        help = "Path to config file"
    )]
    pub config: PathBuf,

    #[arg(
        long,
        help = "Override Bedrock bind address (e.g. 0.0.0.0:19132)"
    )]
    pub bedrock_bind: Option<String>,

    #[arg(
        long,
        help = "Override Java target host"
    )]
    pub java_host: Option<String>,

    #[arg(
        long,
        help = "Override Java target port"
    )]
    pub java_port: Option<u16>,

    #[arg(
        long,
        default_value = "info",
        help = "Log level: trace, debug, info, warn, error"
    )]
    pub log: String,

    #[arg(
        long,
        help = "Disable color in log output"
    )]
    pub no_color: bool,

    #[arg(
        long,
        help = "Print config and exit without starting"
    )]
    pub dry_run: bool,
}
