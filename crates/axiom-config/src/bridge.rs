use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BridgeCfg {
    pub a: bool,
    pub b: u32,
    pub c: u32,
    pub d: bool,
    pub e: u32,
    pub f: bool,
    pub g: u64,
    pub h: u32,
}

impl Default for BridgeCfg {
    fn default() -> Self {
        Self {
            a: true,
            b: 8,
            c: 256,
            d: true,
            e: 30,
            f: true,
            g: 30_000,
            h: 10,
        }
    }
}

impl BridgeCfg {
    pub fn online_mode(&self)     -> bool { self.a }
    pub fn chunk_radius(&self)    -> u32  { self.b }
    pub fn max_chunk_batch(&self) -> u32  { self.c }
    pub fn translate_rp(&self)    -> bool { self.d }
    pub fn keepalive_secs(&self)  -> u32  { self.e }
    pub fn compression(&self)     -> bool { self.f }
    pub fn compress_threshold(&self) -> u64 { self.g }
    pub fn session_timeout(&self) -> u32  { self.h }
}
