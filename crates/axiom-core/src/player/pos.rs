use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct Pos {
    pub x: f64,
    pub y: f64,
    pub z: f64,
    pub yaw:   f32,
    pub pitch: f32,
    pub on_ground: bool,
}

impl Pos {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Self { x, y, z, yaw: 0.0, pitch: 0.0, on_ground: true }
    }

    pub fn with_rot(mut self, yaw: f32, pitch: f32) -> Self {
        self.yaw = yaw; self.pitch = pitch; self
    }

    pub fn distance_sq(&self, other: &Pos) -> f64 {
        let dx = self.x - other.x;
        let dy = self.y - other.y;
        let dz = self.z - other.z;
        dx*dx + dy*dy + dz*dz
    }

    pub fn chunk_pos(&self) -> (i32, i32) {
        ((self.x as i32) >> 4, (self.z as i32) >> 4)
    }
}

impl Default for Pos {
    fn default() -> Self { Self::new(0.0, 64.0, 0.0) }
}
