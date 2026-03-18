use ahash::AHashMap;
use once_cell::sync::OnceCell;
use serde::Deserialize;
use crate::error::{Err, R};

static TABLE: OnceCell<KindTable> = OnceCell::new();

#[derive(Debug, Deserialize)]
struct RawEntry {
    java_id:    i32,
    bedrock_id: i32,
    name:       String,
}

pub struct KindTable {
    a: AHashMap<i32, i32>,
    b: AHashMap<i32, i32>,
    c: AHashMap<i32, String>,
}

impl KindTable {
    fn from_raw(entries: Vec<RawEntry>) -> Self {
        let mut a = AHashMap::with_capacity(entries.len());
        let mut b = AHashMap::with_capacity(entries.len());
        let mut c = AHashMap::with_capacity(entries.len());
        for e in entries {
            a.insert(e.java_id, e.bedrock_id);
            b.insert(e.bedrock_id, e.java_id);
            c.insert(e.java_id, e.name);
        }
        Self { a, b, c }
    }

    pub fn java_to_bedrock(&self, id: i32) -> R<i32> {
        self.a.get(&id).copied().ok_or(Err::NoEntity(id as u32))
    }

    pub fn bedrock_to_java(&self, id: i32) -> R<i32> {
        self.b.get(&id).copied().ok_or(Err::NoEntity(id as u32))
    }

    pub fn name(&self, java_id: i32) -> Option<&str> {
        self.c.get(&java_id).map(|s| s.as_str())
    }
}

pub fn init_from_json(json: &str) -> R<()> {
    let entries: Vec<RawEntry> = serde_json::from_str(json)
        .map_err(|_| Err::NoEntity(0))?;
    TABLE.set(KindTable::from_raw(entries)).ok();
    Ok(())
}

pub fn init_default() {
    TABLE.get_or_init(|| KindTable::from_raw(builtin()));
}

pub fn get() -> R<&'static KindTable> {
    TABLE.get().ok_or(Err::TableNotLoaded("entity_map"))
}

pub fn java_to_bedrock(id: i32) -> R<i32> { get()?.java_to_bedrock(id) }
pub fn bedrock_to_java(id: i32) -> R<i32> { get()?.bedrock_to_java(id) }

fn builtin() -> Vec<RawEntry> {
    vec![
        RawEntry { java_id: 0,  bedrock_id: 1,   name: "player".into() },
        RawEntry { java_id: 37, bedrock_id: 32,  name: "zombie".into() },
        RawEntry { java_id: 60, bedrock_id: 34,  name: "skeleton".into() },
        RawEntry { java_id: 19, bedrock_id: 33,  name: "creeper".into() },
        RawEntry { java_id: 77, bedrock_id: 35,  name: "spider".into() },
        RawEntry { java_id: 27, bedrock_id: 38,  name: "enderman".into() },
        RawEntry { java_id: 63, bedrock_id: 12,  name: "pig".into() },
        RawEntry { java_id: 15, bedrock_id: 11,  name: "cow".into() },
        RawEntry { java_id: 73, bedrock_id: 13,  name: "sheep".into() },
        RawEntry { java_id: 18, bedrock_id: 10,  name: "chicken".into() },
        RawEntry { java_id: 41, bedrock_id: 20,  name: "iron_golem".into() },
        RawEntry { java_id: 88, bedrock_id: 15,  name: "villager".into() },
        RawEntry { java_id: 47, bedrock_id: 23,  name: "horse".into() },
        RawEntry { java_id: 101, bedrock_id: 31, name: "item".into() },
        RawEntry { java_id: 102, bedrock_id: 64, name: "xp_orb".into() },
        RawEntry { java_id: 103, bedrock_id: 65, name: "tnt".into() },
        RawEntry { java_id: 106, bedrock_id: 80, name: "arrow".into() },
        RawEntry { java_id: 108, bedrock_id: 81, name: "snowball".into() },
        RawEntry { java_id: 110, bedrock_id: 87, name: "ender_pearl".into() },
    ]
}
