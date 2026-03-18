use bytes::Bytes;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Edition { Java, Bedrock }

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Dir { C2S, S2C }

#[derive(Debug, Clone)]
pub struct Envelope {
    pub a: Edition,
    pub b: Dir,
    pub c: i32,
    pub d: Bytes,
}

impl Envelope {
    pub fn new(edition: Edition, dir: Dir, id: i32, data: Bytes) -> Self {
        Self { a: edition, b: dir, c: id, d: data }
    }

    pub fn java_c2s(id: i32, data: Bytes) -> Self {
        Self::new(Edition::Java, Dir::C2S, id, data)
    }

    pub fn java_s2c(id: i32, data: Bytes) -> Self {
        Self::new(Edition::Java, Dir::S2C, id, data)
    }

    pub fn bedrock_c2s(id: i32, data: Bytes) -> Self {
        Self::new(Edition::Bedrock, Dir::C2S, id, data)
    }

    pub fn bedrock_s2c(id: i32, data: Bytes) -> Self {
        Self::new(Edition::Bedrock, Dir::S2C, id, data)
    }

    pub fn edition(&self) -> Edition { self.a }
    pub fn dir(&self)     -> Dir     { self.b }
    pub fn id(&self)      -> i32     { self.c }
    pub fn data(&self)    -> &Bytes  { &self.d }
}
