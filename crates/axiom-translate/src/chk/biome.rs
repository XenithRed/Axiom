use ahash::AHashMap;
use once_cell::sync::OnceCell;
use serde::Deserialize;
use crate::error::{Err, R};

static TABLE: OnceCell<BiomeTable> = OnceCell::new();

#[derive(Debug, Deserialize)]
struct RawEntry {
    java_id:    u32,
    bedrock_id: u32,
    name:       String,
}

pub struct BiomeTable {
    a: AHashMap<u32, u32>,
    b: AHashMap<u32, u32>,
}

impl BiomeTable {
    fn from_raw(entries: Vec<RawEntry>) -> Self {
        let mut a = AHashMap::with_capacity(entries.len());
        let mut b = AHashMap::with_capacity(entries.len());
        for e in entries {
            a.insert(e.java_id, e.bedrock_id);
            b.entry(e.bedrock_id).or_insert(e.java_id);
        }
        Self { a, b }
    }

    pub fn java_to_bedrock(&self, id: u32) -> u32 {
        self.a.get(&id).copied().unwrap_or(1)
    }

    pub fn bedrock_to_java(&self, id: u32) -> u32 {
        self.b.get(&id).copied().unwrap_or(1)
    }
}

pub fn init_default() {
    TABLE.get_or_init(|| BiomeTable::from_raw(builtin()));
}

pub fn init_from_json(json: &str) -> R<()> {
    let entries: Vec<RawEntry> = serde_json::from_str(json)
        .map_err(|_| Err::Chunk("biome json".into()))?;
    TABLE.set(BiomeTable::from_raw(entries)).ok();
    Ok(())
}

pub fn java_to_bedrock(id: u32) -> u32 {
    TABLE.get().map(|t| t.java_to_bedrock(id)).unwrap_or(1)
}

pub fn bedrock_to_java(id: u32) -> u32 {
    TABLE.get().map(|t| t.bedrock_to_java(id)).unwrap_or(1)
}

fn builtin() -> Vec<RawEntry> {
    vec![
        RawEntry { java_id: 0,  bedrock_id: 0,  name: "ocean".into() },
        RawEntry { java_id: 1,  bedrock_id: 1,  name: "plains".into() },
        RawEntry { java_id: 2,  bedrock_id: 2,  name: "desert".into() },
        RawEntry { java_id: 3,  bedrock_id: 3,  name: "mountains".into() },
        RawEntry { java_id: 4,  bedrock_id: 4,  name: "forest".into() },
        RawEntry { java_id: 5,  bedrock_id: 5,  name: "taiga".into() },
        RawEntry { java_id: 6,  bedrock_id: 6,  name: "swamp".into() },
        RawEntry { java_id: 7,  bedrock_id: 7,  name: "river".into() },
        RawEntry { java_id: 8,  bedrock_id: 8,  name: "nether_wastes".into() },
        RawEntry { java_id: 9,  bedrock_id: 9,  name: "the_end".into() },
        RawEntry { java_id: 10, bedrock_id: 10, name: "frozen_ocean".into() },
        RawEntry { java_id: 12, bedrock_id: 12, name: "snowy_plains".into() },
        RawEntry { java_id: 16, bedrock_id: 16, name: "badlands".into() },
        RawEntry { java_id: 21, bedrock_id: 21, name: "jungle".into() },
        RawEntry { java_id: 27, bedrock_id: 27, name: "birch_forest".into() },
        RawEntry { java_id: 30, bedrock_id: 30, name: "dark_forest".into() },
    ]
}
