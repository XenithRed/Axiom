use thiserror::Error;

#[derive(Debug, Error)]
pub enum Err {
    #[error("unexpected eof")]
    Eof,
    #[error("io: {0}")]
    Io(#[from] std::io::Error),
    #[error("varint overflow")]
    VarOverflow,
    #[error("invalid packet id 0x{0:02x} in state {1}")]
    BadId(i32, &'static str),
    #[error("string too long: {0} > {1}")]
    StrTooLong(usize, usize),
    #[error("invalid utf-8: {0}")]
    Utf8(#[from] std::string::FromUtf8Error),
    #[error("nbt: {0}")]
    Nbt(String),
    #[error("compression: {0}")]
    Compress(String),
    #[error("crypto: {0}")]
    Crypto(String),
    #[error("rsa: {0}")]
    Rsa(String),
    #[error("auth: {0}")]
    Auth(String),
    #[error("json: {0}")]
    Json(#[from] serde_json::Error),
    #[error("frame too large: {0}")]
    FrameTooLarge(usize),
    #[error("connection closed")]
    Closed,
}

pub type R<T> = Result<T, Err>;
