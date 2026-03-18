use serde::Deserialize;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct JavaState(pub u32);

impl JavaState {
    pub const AIR: Self = Self(0);
    pub fn id(self) -> u32 { self.0 }
    pub fn is_air(self) -> bool { self.0 == 0 }
}

#[derive(Debug, Clone, Deserialize)]
pub struct JavaEntry {
    pub a: u32,
    pub b: String,
    pub c: Vec<(String, String)>,
}

impl JavaEntry {
    pub fn state_id(&self) -> JavaState { JavaState(self.a) }
    pub fn block_name(&self) -> &str { &self.b }
    pub fn props(&self) -> &[(String, String)] { &self.c }
}
