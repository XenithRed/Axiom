use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServerCfg {
    pub a: String,
    pub b: String,
    pub c: u16,
    pub d: String,
    pub e: u32,
    pub f: bool,
    pub g: Option<String>,
    pub h: Option<u16>,
}

impl Default for ServerCfg {
    fn default() -> Self {
        Self {
            a: "0.0.0.0:19132".into(),
            b: "127.0.0.1".into(),
            c: 25565,
            d: "Axiom Bridge".into(),
            e: 100,
            f: false,
            g: None,
            h: None,
        }
    }
}

impl ServerCfg {
    pub fn bedrock_addr(&self) -> &str { &self.a }
    pub fn java_host(&self)    -> &str { &self.b }
    pub fn java_port(&self)    -> u16  { self.c }
    pub fn motd(&self)         -> &str { &self.d }
    pub fn max_players(&self)  -> u32  { self.e }
    pub fn prometheus(&self)   -> bool { self.f }
    pub fn metrics_addr(&self) -> Option<&str> { self.g.as_deref() }
    pub fn metrics_port(&self) -> Option<u16>  { self.h }
}
