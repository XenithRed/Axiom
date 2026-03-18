use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum GameMode { Survival, Creative, Adventure, Spectator }

impl GameMode {
    pub fn from_i32(n: i32) -> Self {
        match n { 1 => Self::Creative, 2 => Self::Adventure, 3 => Self::Spectator, _ => Self::Survival }
    }
    pub fn to_i32(self) -> i32 {
        match self { Self::Survival => 0, Self::Creative => 1, Self::Adventure => 2, Self::Spectator => 3 }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ConnState { Handshake, Login, Config, Play, Closing }

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Stats {
    pub a: f32,
    pub b: i32,
    pub c: f32,
    pub d: f32,
    pub e: i32,
    pub f: i32,
}

impl Default for Stats {
    fn default() -> Self {
        Self { a: 20.0, b: 20, c: 20.0, d: 0.0, e: 0, f: 0 }
    }
}

impl Stats {
    pub fn health(&self) -> f32     { self.a }
    pub fn food(&self) -> i32       { self.b }
    pub fn saturation(&self) -> f32 { self.c }
    pub fn absorption(&self) -> f32 { self.d }
    pub fn xp_level(&self) -> i32   { self.e }
    pub fn xp_progress(&self) -> i32 { self.f }

    pub fn set_health(&mut self, v: f32) { self.a = v.clamp(0.0, 20.0); }
    pub fn set_food(&mut self, v: i32)   { self.b = v.clamp(0, 20); }
    pub fn is_alive(&self) -> bool       { self.a > 0.0 }
}
