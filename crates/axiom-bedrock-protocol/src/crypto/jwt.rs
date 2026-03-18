use base64::{engine::general_purpose::URL_SAFE_NO_PAD as B64, Engine};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use crate::error::{Err, R};

#[derive(Debug, Serialize, Deserialize)]
pub struct ChainData {
    pub chain: Vec<String>,
}

#[derive(Debug, Deserialize)]
pub struct IdentityClaims {
    #[serde(rename = "extraData")]
    pub a: Option<ExtraData>,
    #[serde(rename = "identityPublicKey")]
    pub b: String,
}

#[derive(Debug, Deserialize)]
pub struct ExtraData {
    #[serde(rename = "XUID")]
    pub a: String,
    #[serde(rename = "displayName")]
    pub b: String,
    #[serde(rename = "identity")]
    pub c: String,
}

pub fn decode_chain(raw: &[u8]) -> R<ChainData> {
    serde_json::from_slice(raw).map_err(|e| Err::Jwt(e.to_string()))
}

pub fn extract_identity(chain: &ChainData) -> R<IdentityClaims> {
    let last = chain.chain.last().ok_or_else(|| Err::Jwt("empty chain".into()))?;
    decode_payload(last)
}

pub fn decode_payload<T: for<'de> Deserialize<'de>>(jwt: &str) -> R<T> {
    let parts: Vec<&str> = jwt.splitn(3, '.').collect();
    if parts.len() < 2 { return Err(Err::Jwt("malformed jwt".into())); }
    let raw = B64.decode(parts[1]).map_err(|e| Err::Jwt(e.to_string()))?;
    serde_json::from_slice(&raw).map_err(|e| Err::Jwt(e.to_string()))
}

pub fn build_server_chain(pubkey_b64: &str, privkey_pem: &str) -> R<ChainData> {
    let payload = serde_json::json!({
        "identityPublicKey": pubkey_b64,
        "nbf": now_secs() - 60,
        "exp": now_secs() + 86400,
        "iat": now_secs(),
    });
    let tok = sign_jwt(&payload, privkey_pem)?;
    Ok(ChainData { chain: vec![tok] })
}

pub fn sign_jwt(payload: &Value, _privkey_pem: &str) -> R<String> {
    let h = B64.encode(br#"{"alg":"ES384","x5u":""}"#);
    let p = B64.encode(payload.to_string().as_bytes());
    Ok(format!("{h}.{p}.placeholder_sig"))
}

fn now_secs() -> u64 {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs()
}
