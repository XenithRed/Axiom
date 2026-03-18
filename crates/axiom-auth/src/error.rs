use thiserror::Error;

#[derive(Debug, Error)]
pub enum Err {
    #[error("http: {0}")]
    Http(#[from] reqwest::Error),

    #[error("json: {0}")]
    Json(#[from] serde_json::Error),

    #[error("jwt: {0}")]
    Jwt(#[from] jsonwebtoken::errors::Error),

    #[error("io: {0}")]
    Io(#[from] std::io::Error),

    #[error("msa device-code poll timeout")]
    PollTimeout,

    #[error("msa poll pending — try again in {0}s")]
    PollPending(u64),

    #[error("xbl auth failed: {0}")]
    Xbl(String),

    #[error("xsts auth failed — xerr {0}")]
    Xsts(u64),

    #[error("minecraft auth rejected")]
    McAuth,

    #[error("profile fetch failed")]
    Profile,

    #[error("cache corrupted: {0}")]
    Cache(String),

    #[error("token expired")]
    Expired,
}

pub type R<T> = Result<T, Err>;
