use serde::{Deserialize, Serialize};
use std::path::Path;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Cfg {
    pub a: ServerCfg,
    pub b: BridgeCfg,
    pub c: RpCfg,
    pub d: LogCfg,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServerCfg {
    pub a: String,
    pub b: String,
    pub c: u16,
    pub d: String,
    pub e: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BridgeCfg {
    pub a: bool,
    pub b: u32,
    pub c: u32,
    pub d: bool,
    pub e: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RpCfg {
    pub a: bool,
    pub b: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LogCfg {
    pub a: String,
    pub b: bool,
}

impl Default for Cfg {
    fn default() -> Self {
        Self {
            a: ServerCfg {
                a: "0.0.0.0:19132".into(),
                b: "127.0.0.1".into(),
                c: 25565,
                d: "Axiom Bridge".into(),
                e: 100,
            },
            b: BridgeCfg {
                a: true,
                b: 8,
                c: 256,
                d: true,
                e: 30,
            },
            c: RpCfg {
                a: true,
                b: ".axiom/rp-cache".into(),
            },
            d: LogCfg {
                a: "info".into(),
                b: true,
            },
        }
    }
}

impl Cfg {
    pub fn load(path: &Path) -> Self {
        if !path.exists() {
            tracing::warn!("config file not found at {}, using defaults", path.display());
            return Self::default();
        }
        match std::fs::read_to_string(path) {
            Ok(raw) => toml::from_str(&raw).unwrap_or_else(|e| {
                tracing::warn!("config parse error: {e}, using defaults");
                Self::default()
            }),
            Err(e) => {
                tracing::warn!("config read error: {e}, using defaults");
                Self::default()
            }
        }
    }

    pub fn apply_overrides(&mut self, cli: &crate::cli::Cli) {
        if let Some(ref b) = cli.bedrock_bind { self.a.a = b.clone(); }
        if let Some(ref h) = cli.java_host    { self.a.b = h.clone(); }
        if let Some(p) = cli.java_port         { self.a.c = p; }
        self.d.a = cli.log.clone();
        if cli.no_color { self.d.b = false; }
    }

    pub fn bedrock_addr(&self) -> &str { &self.a.a }
    pub fn java_host(&self)    -> &str { &self.a.b }
    pub fn java_port(&self)    -> u16  { self.a.c }
    pub fn motd(&self)         -> &str { &self.a.d }
    pub fn max_players(&self)  -> u32  { self.a.e }
    pub fn online_mode(&self)  -> bool { self.b.a }
    pub fn chunk_radius(&self) -> u32  { self.b.b }
}
