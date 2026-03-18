use thiserror::Error;

#[derive(Debug, Error)]
pub enum Err {
    #[error("io: {0}")]
    Io(#[from] std::io::Error),
    #[error("entity not found: {0}")]
    NoEntity(u32),
    #[error("chunk not loaded: ({0}, {1})")]
    NoChunk(i32, i32),
    #[error("block out of range: ({0}, {1}, {2})")]
    OutOfRange(i32, i32, i32),
    #[error("invalid block state: {0}")]
    BadState(u32),
    #[error("invalid section y: {0}")]
    BadSection(i32),
    #[error("inventory slot out of range: {0}")]
    BadSlot(usize),
    #[error("connection closed")]
    Closed,
    #[error("state machine: {0}")]
    Agsm(String),
    #[error("send failed")]
    Send,
}

pub type R<T> = Result<T, Err>;
