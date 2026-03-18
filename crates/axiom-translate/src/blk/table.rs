use ahash::AHashMap;
use once_cell::sync::OnceCell;
use serde::Deserialize;
use super::{
    bedrock::BedrockState,
    java::JavaState,
};
use crate::error::{Err, R};

static TABLE: OnceCell<Table> = OnceCell::new();

#[derive(Debug, Deserialize)]
struct RawEntry {
    java_id:     u32,
    runtime_id:  u32,
    extra_data:  u16,
}

pub struct Table {
    a: Vec<BedrockState>,
    b: AHashMap<BedrockState, JavaState>,
}

impl Table {
    fn from_raw(entries: Vec<RawEntry>) -> Self {
        let cap = entries.iter().map(|e| e.java_id as usize + 1).max().unwrap_or(0);
        let mut a = vec![BedrockState::AIR; cap];
        let mut b = AHashMap::with_capacity(entries.len());

        for e in entries {
            let js = JavaState(e.java_id);
            let bs = BedrockState::new(e.runtime_id, e.extra_data);
            if (e.java_id as usize) < a.len() {
                a[e.java_id as usize] = bs;
            }
            b.entry(bs).or_insert(js);
        }

        Self { a, b }
    }

    #[inline(always)]
    pub fn to_bedrock(&self, s: JavaState) -> R<BedrockState> {
        self.a.get(s.0 as usize)
            .copied()
            .ok_or(Err::NoJavaBlock(s.0))
    }

    #[inline(always)]
    pub fn to_java(&self, s: BedrockState) -> R<JavaState> {
        self.b.get(&s)
            .copied()
            .ok_or(Err::NoBedrockBlock(s.a))
    }
}

pub fn init_from_json(json: &str) -> R<()> {
    let entries: Vec<RawEntry> = serde_json::from_str(json)
        .map_err(|e| Err::NoJavaBlock(0))?;
    let t = Table::from_raw(entries);
    TABLE.set(t).ok();
    Ok(())
}

pub fn init_default() {
    TABLE.get_or_init(|| {
        Table::from_raw(builtin_entries())
    });
}

pub fn get() -> R<&'static Table> {
    TABLE.get().ok_or(Err::TableNotLoaded("block_states"))
}

#[inline(always)]
pub fn java_to_bedrock(s: JavaState) -> R<BedrockState> {
    get()?.to_bedrock(s)
}

#[inline(always)]
pub fn bedrock_to_java(s: BedrockState) -> R<JavaState> {
    get()?.to_java(s)
}

fn builtin_entries() -> Vec<RawEntry> {
    vec![
        RawEntry { java_id: 0,  runtime_id: 0,   extra_data: 0 },
        RawEntry { java_id: 1,  runtime_id: 1,   extra_data: 0 },
        RawEntry { java_id: 2,  runtime_id: 2,   extra_data: 0 },
        RawEntry { java_id: 8,  runtime_id: 8,   extra_data: 0 },
        RawEntry { java_id: 9,  runtime_id: 9,   extra_data: 0 },
        RawEntry { java_id: 10, runtime_id: 10,  extra_data: 0 },
        RawEntry { java_id: 11, runtime_id: 11,  extra_data: 0 },
        RawEntry { java_id: 12, runtime_id: 12,  extra_data: 0 },
        RawEntry { java_id: 13, runtime_id: 13,  extra_data: 0 },
        RawEntry { java_id: 14, runtime_id: 14,  extra_data: 0 },
        RawEntry { java_id: 15, runtime_id: 15,  extra_data: 0 },
        RawEntry { java_id: 16, runtime_id: 16,  extra_data: 0 },
        RawEntry { java_id: 17, runtime_id: 17,  extra_data: 0 },
        RawEntry { java_id: 18, runtime_id: 18,  extra_data: 0 },
        RawEntry { java_id: 19, runtime_id: 19,  extra_data: 0 },
        RawEntry { java_id: 20, runtime_id: 20,  extra_data: 0 },
    ]
}
