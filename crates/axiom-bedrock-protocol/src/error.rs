use thiserror::Error;

#[derive(Debug, Error)]
pub enum Err {
    #[error("unexpected eof")]
    Eof,
    #[error("io: {0}")]
    Io(#[from] std::io::Error),
    #[error("invalid packet id: 0x{0:02x}")]
    BadId(u8),
    #[error("invalid reliability: {0}")]
    BadRel(u8),
    #[error("fragment overflow: id={0} idx={1} size={2}")]
    FragOverflow(u16, u32, u32),
    #[error("nbt: {0}")]
    Nbt(String),
    #[error("crypto: {0}")]
    Crypto(String),
    #[error("jwt: {0}")]
    Jwt(String),
    #[error("zlib: {0}")]
    Zlib(String),
    #[error("order channel out of range: {0}")]
    BadChannel(u8),
    #[error("buffer too large: {0} bytes")]
    TooLarge(usize),
    #[error("send queue full")]
    QueueFull,
    #[error("connection closed")]
    Closed,
}

pub type R<T> = Result<T, Err>;
