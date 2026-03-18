use ahash::AHashMap;
use once_cell::sync::OnceCell;
use serde::Deserialize;
use crate::error::{Err, R};

static TABLE: OnceCell<ItemTable> = OnceCell::new();

#[derive(Debug, Deserialize)]
struct RawEntry {
    java_id:    i32,
    bedrock_id: i32,
    bedrock_damage: i16,
    name:       String,
}

pub struct ItemTable {
    a: AHashMap<i32, (i32, i16)>,
    b: AHashMap<i32, i32>,
    c: AHashMap<i32, String>,
}

impl ItemTable {
    fn from_raw(entries: Vec<RawEntry>) -> Self {
        let mut a = AHashMap::with_capacity(entries.len());
        let mut b = AHashMap::with_capacity(entries.len());
        let mut c = AHashMap::with_capacity(entries.len());
        for e in entries {
            a.insert(e.java_id, (e.bedrock_id, e.bedrock_damage));
            b.entry(e.bedrock_id).or_insert(e.java_id);
            c.insert(e.java_id, e.name);
        }
        Self { a, b, c }
    }

    pub fn java_to_bedrock(&self, id: i32) -> R<(i32, i16)> {
        self.a.get(&id).copied().ok_or(Err::NoItem(id))
    }

    pub fn bedrock_to_java(&self, id: i32) -> R<i32> {
        self.b.get(&id).copied().ok_or(Err::NoItem(id))
    }

    pub fn name(&self, java_id: i32) -> Option<&str> {
        self.c.get(&java_id).map(|s| s.as_str())
    }
}

pub fn init_from_json(json: &str) -> R<()> {
    let entries: Vec<RawEntry> = serde_json::from_str(json)
        .map_err(|_| Err::NoItem(0))?;
    TABLE.set(ItemTable::from_raw(entries)).ok();
    Ok(())
}

pub fn init_default() {
    TABLE.get_or_init(|| ItemTable::from_raw(builtin()));
}

pub fn get() -> R<&'static ItemTable> {
    TABLE.get().ok_or(Err::TableNotLoaded("item_map"))
}

pub fn java_to_bedrock(id: i32) -> R<(i32, i16)> { get()?.java_to_bedrock(id) }
pub fn bedrock_to_java(id: i32) -> R<i32>        { get()?.bedrock_to_java(id) }

fn builtin() -> Vec<RawEntry> {
    vec![
        RawEntry { java_id: 0,   bedrock_id: 0,   bedrock_damage: 0, name: "air".into() },
        RawEntry { java_id: 1,   bedrock_id: 1,   bedrock_damage: 0, name: "stone".into() },
        RawEntry { java_id: 290, bedrock_id: 290, bedrock_damage: 0, name: "wooden_shovel".into() },
        RawEntry { java_id: 291, bedrock_id: 291, bedrock_damage: 0, name: "wooden_pickaxe".into() },
        RawEntry { java_id: 292, bedrock_id: 292, bedrock_damage: 0, name: "wooden_axe".into() },
        RawEntry { java_id: 256, bedrock_id: 256, bedrock_damage: 0, name: "iron_shovel".into() },
        RawEntry { java_id: 257, bedrock_id: 257, bedrock_damage: 0, name: "iron_pickaxe".into() },
        RawEntry { java_id: 258, bedrock_id: 258, bedrock_damage: 0, name: "iron_axe".into() },
        RawEntry { java_id: 261, bedrock_id: 261, bedrock_damage: 0, name: "bow".into() },
        RawEntry { java_id: 262, bedrock_id: 262, bedrock_damage: 0, name: "arrow".into() },
        RawEntry { java_id: 364, bedrock_id: 364, bedrock_damage: 0, name: "cooked_beef".into() },
        RawEntry { java_id: 365, bedrock_id: 365, bedrock_damage: 0, name: "cooked_chicken".into() },
        RawEntry { java_id: 297, bedrock_id: 297, bedrock_damage: 0, name: "bread".into() },
        RawEntry { java_id: 345, bedrock_id: 345, bedrock_damage: 0, name: "compass".into() },
        RawEntry { java_id: 346, bedrock_id: 346, bedrock_damage: 0, name: "fishing_rod".into() },
    ]
}
