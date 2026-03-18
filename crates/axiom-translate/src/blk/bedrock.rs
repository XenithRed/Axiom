use serde::Deserialize;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct BedrockState {
    pub a: u32,
    pub b: u16,
}

impl BedrockState {
    pub const AIR: Self = Self { a: 0, b: 0 };

    pub fn new(runtime_id: u32, extra: u16) -> Self {
        Self { a: runtime_id, b: extra }
    }

    pub fn runtime_id(self) -> u32 { self.a }
    pub fn extra_data(self) -> u16 { self.b }
    pub fn is_air(self) -> bool { self.a == 0 }
}

#[derive(Debug, Clone, Deserialize)]
pub struct BedrockEntry {
    pub a: u32,
    pub b: String,
    pub c: u16,
}

impl BedrockEntry {
    pub fn runtime_id(&self) -> u32 { self.a }
    pub fn block_name(&self) -> &str { &self.b }
    pub fn extra_data(&self) -> u16 { self.c }
    pub fn state(&self) -> BedrockState {
        BedrockState::new(self.a, self.c)
    }
}
