use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum MetaVal {
    Byte(i8),
    Int(i32),
    Float(f32),
    Str(String),
    Bool(bool),
    Vec3(f32, f32, f32),
    OptUuid(Option<u128>),
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Meta {
    a: Vec<(u8, MetaVal)>,
}

impl Meta {
    pub fn new() -> Self { Self::default() }

    pub fn set(&mut self, idx: u8, val: MetaVal) {
        if let Some(e) = self.a.iter_mut().find(|(i, _)| *i == idx) {
            e.1 = val;
        } else {
            self.a.push((idx, val));
        }
    }

    pub fn get(&self, idx: u8) -> Option<&MetaVal> {
        self.a.iter().find(|(i, _)| *i == idx).map(|(_, v)| v)
    }

    pub fn remove(&mut self, idx: u8) {
        self.a.retain(|(i, _)| *i != idx);
    }

    pub fn iter(&self) -> impl Iterator<Item = (u8, &MetaVal)> {
        self.a.iter().map(|(i, v)| (*i, v))
    }

    pub fn is_on_fire(&self) -> bool {
        self.get(0).and_then(|v| if let MetaVal::Byte(n) = v { Some(*n & 0x01 != 0) } else { None }).unwrap_or(false)
    }

    pub fn is_invisible(&self) -> bool {
        self.get(0).and_then(|v| if let MetaVal::Byte(n) = v { Some(*n & 0x20 != 0) } else { None }).unwrap_or(false)
    }

    pub fn is_sneaking(&self) -> bool {
        self.get(0).and_then(|v| if let MetaVal::Byte(n) = v { Some(*n & 0x02 != 0) } else { None }).unwrap_or(false)
    }

    pub fn custom_name(&self) -> Option<&str> {
        self.get(2).and_then(|v| if let MetaVal::Str(s) = v { Some(s.as_str()) } else { None })
    }

    pub fn health(&self) -> Option<f32> {
        self.get(9).and_then(|v| if let MetaVal::Float(f) = v { Some(*f) } else { None })
    }
}
