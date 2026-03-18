use thiserror::Error;

#[derive(Debug, Error)]
pub enum Err {
    #[error("io: {0}")]
    Io(#[from] std::io::Error),
    #[error("json: {0}")]
    Json(#[from] serde_json::Error),
    #[error("zip: {0}")]
    Zip(#[from] zip::result::ZipError),
    #[error("model: {0}")]
    Model(String),
    #[error("texture: {0}")]
    Tex(String),
    #[error("sound: {0}")]
    Sound(String),
    #[error("pack not found: {0}")]
    NotFound(String),
    #[error("unsupported format: {0}")]
    Format(String),
}

pub type R<T> = Result<T, Err>;
