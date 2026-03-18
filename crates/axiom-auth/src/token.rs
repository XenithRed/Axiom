use serde::{Deserialize, Serialize};
use std::time::{Duration, SystemTime, UNIX_EPOCH};

#[inline]
pub fn now_secs() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or(Duration::ZERO)
        .as_secs()
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MsaTok {
    pub a: String,   // access_token
    pub b: String,   // refresh_token
    pub c: u64,     // expires_at  (unix secs)
}

impl MsaTok {
    pub fn live(&self) -> bool { now_secs() < self.c.saturating_sub(60) }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct XblTok {
    pub a: String,   // Token
    pub b: String,   // UserHash (uhs)
    pub c: u64,     // expires_at
}

impl XblTok {
    pub fn live(&self) -> bool { now_secs() < self.c.saturating_sub(60) }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct XstsTok {
    pub a: String,   // Token
    pub b: String,   // UserHash (uhs)
    pub c: u64,     // expires_at
}

impl XstsTok {
    pub fn live(&self) -> bool { now_secs() < self.c.saturating_sub(60) }

    pub fn hdr(&self) -> String {
        format!("XBL3.0 x={};{}", self.b, self.a)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct McTok {
    pub a: String,   // access_token  (JWT)
    pub b: u64,      // expires_at
}

impl McTok {
    pub fn live(&self) -> bool { now_secs() < self.b.saturating_sub(60) }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Profile {
    pub a: String,   // uuid  (without dashes)
    pub b: String,   // name
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Session {
    pub msa:     MsaTok,
    pub xbl:     XblTok,
    pub xsts:    XstsTok,
    pub mc:      McTok,
    pub profile: Profile,
}

impl Session {
    pub fn live(&self) -> bool {
        self.msa.live() && self.xbl.live() && self.xsts.live() && self.mc.live()
    }
}
