use crate::{
    error::{Err, R},
    token::{now_secs, McTok, Profile},
};
use reqwest::Client;
use serde::{Deserialize, Serialize};
use tracing::debug;

const A: &str = "https://api.minecraftservices.com/authentication/login_with_xbox";
const B: &str = "https://api.minecraftservices.com/minecraft/profile";

#[derive(Serialize)]
struct AuthReq<'x> {
    #[serde(rename = "identityToken")]
    a: &'x str,
    #[serde(rename = "ensureLegacyEnabled")]
    b: bool,
}

#[derive(Deserialize)]
struct AuthResp {
    access_token: String,
    expires_in:   u64,
}

#[derive(Deserialize)]
struct ProfResp {
    id:   String,
    name: String,
}

pub async fn login(cx: &Client, xsts_hdr: &str) -> R<McTok> {
    let jh = cx
        .post(A)
        .json(&AuthReq { a: xsts_hdr, b: true })
        .send()
        .await?
        .error_for_status()
        .map_err(|_| Err::McAuth)?
        .json::<AuthResp>()
        .await?;

    debug!(expires_in = jh.expires_in, "minecraft access token issued");
    Ok(McTok { a: jh.access_token, b: now_secs() + jh.expires_in })
}

pub async fn profile(cx: &Client, mc_token: &str) -> R<Profile> {
    let jh = cx
        .get(B)
        .bearer_auth(mc_token)
        .send()
        .await?
        .error_for_status()
        .map_err(|_| Err::Profile)?
        .json::<ProfResp>()
        .await?;

    debug!(uuid = %jh.id, name = %jh.name, "profile fetched");
    Ok(Profile { a: jh.id, b: jh.name })
}
