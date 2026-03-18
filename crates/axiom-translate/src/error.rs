use thiserror::Error;

#[derive(Debug, Error)]
pub enum Err {
    #[error("block state not found: java={0}")]
    NoJavaBlock(u32),
    #[error("block state not found: bedrock runtime_id={0}")]
    NoBedrockBlock(u32),
    #[error("entity kind unknown: {0}")]
    NoEntity(u32),
    #[error("item id unknown: java={0}")]
    NoItem(i32),
    #[error("nbt: {0}")]
    Nbt(String),
    #[error("chunk: {0}")]
    Chunk(String),
    #[error("table not loaded: {0}")]
    TableNotLoaded(&'static str),
}

pub type R<T> = Result<T, Err>;
