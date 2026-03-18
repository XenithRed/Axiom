pub mod bridge;
pub mod log;
pub mod rp;
pub mod server;

pub use bridge::BridgeCfg;
pub use log::LogCfg;
pub use rp::RpCfg;
pub use server::ServerCfg;

use serde::{Deserialize, Serialize};
use std::path::Path;

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Config {
    pub server: ServerCfg,
    pub bridge: BridgeCfg,
    pub rp:     RpCfg,
    pub log:    LogCfg,
}

#[derive(Debug, thiserror::Error)]
pub enum Err {
    #[error("io: {0}")]
    Io(#[from] std::io::Error),
    #[error("parse: {0}")]
    Parse(String),
}

pub type R<T> = Result<T, Err>;

impl Config {
    pub fn load(path: &Path) -> R<Self> {
        if !path.exists() {
            tracing::warn!("config not found at {}, using defaults", path.display());
            return Ok(Self::default());
        }
        let raw = std::fs::read_to_string(path)?;
        toml::from_str(&raw).map_err(|e| Err::Parse(e.to_string()))
    }

    pub fn load_or_default(path: &Path) -> Self {
        Self::load(path).unwrap_or_else(|e| {
            tracing::warn!("config error: {e}, using defaults");
            Self::default()
        })
    }

    pub fn to_toml(&self) -> R<String> {
        toml::to_string_pretty(self).map_err(|e| Err::Parse(e.to_string()))
    }

    pub fn save(&self, path: &Path) -> R<()> {
        if let Some(parent) = path.parent() {
            std::fs::create_dir_all(parent)?;
        }
        std::fs::write(path, self.to_toml()?)?;
        Ok(())
    }
}
