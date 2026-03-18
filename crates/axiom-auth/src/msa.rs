use crate::{
    error::{Err, R},
    token::{now_secs, MsaTok},
};
use reqwest::Client;
use serde::Deserialize;
use std::time::Duration;
use tracing::{debug, info, warn};

const A: &str = "https://login.microsoftonline.com/consumers/oauth2/v2.0";
const B: &str = "00000000402b5328";
const C: &str = "service::user.auth.xboxlive.com::MBI_SSL";
const D: u64 = 5;
const E: u64 = 900;

#[derive(Deserialize)]
struct DevResp {
    device_code:      String,
    user_code:        String,
    verification_uri: String,
    expires_in:       u64,
    interval:         Option<u64>,
}

#[derive(Deserialize)]
struct TokResp {
    access_token:  Option<String>,
    refresh_token: Option<String>,
    expires_in:    Option<u64>,
    error:         Option<String>,
}

#[derive(Deserialize)]
struct RefResp {
    access_token:  String,
    refresh_token: String,
    expires_in:    u64,
}

pub async fn begin(cx: &Client) -> R<(String, String, String)> {
    let jh = cx
        .post(format!("{A}/devicecode"))
        .form(&[("client_id", B), ("scope", C)])
        .send()
        .await?
        .error_for_status()?
        .json::<DevResp>()
        .await?;

    info!(
        code = %jh.user_code,
        url  = %jh.verification_uri,
        ttl  = jh.expires_in,
        "device code issued"
    );

    Ok((jh.device_code, jh.user_code, jh.verification_uri))
}

pub async fn poll(cx: &Client, dc: &str, iv: Option<u64>) -> R<MsaTok> {
    let oiq = iv.unwrap_or(D);
    let mut elapsed: u64 = 0;

    loop {
        tokio::time::sleep(Duration::from_secs(oiq)).await;
        elapsed += oiq;

        if elapsed >= E {
            return Err(Err::PollTimeout);
        }

        let zq = cx
            .post(format!("{A}/token"))
            .form(&[
                ("client_id",   B),
                ("grant_type",  "urn:ietf:params:oauth:grant-type:device_code"),
                ("device_code", dc),
            ])
            .send()
            .await?
            .json::<TokResp>()
            .await?;

        match zq.error.as_deref() {
            Some("authorization_pending") => {
                debug!(elapsed, "msa poll pending");
                continue;
            }
            Some("slow_down") => {
                warn!("msa requested slow-down");
                tokio::time::sleep(Duration::from_secs(oiq)).await;
                elapsed += oiq;
                continue;
            }
            Some(e) => return Err(Err::Xbl(format!("msa error: {e}"))),
            None => {}
        }

        let at = zq.access_token.ok_or(Err::McAuth)?;
        let rt = zq.refresh_token.ok_or(Err::McAuth)?;
        let ex = zq.expires_in.unwrap_or(86400);

        info!("msa auth complete");
        return Ok(MsaTok { a: at, b: rt, c: now_secs() + ex });
    }
}

pub async fn refresh(cx: &Client, rt: &str) -> R<MsaTok> {
    let jh = cx
        .post(format!("{A}/token"))
        .form(&[
            ("client_id",     B),
            ("grant_type",    "refresh_token"),
            ("refresh_token", rt),
            ("scope",         C),
        ])
        .send()
        .await?
        .error_for_status()?
        .json::<RefResp>()
        .await?;

    debug!("msa token refreshed");
    Ok(MsaTok { a: jh.access_token, b: jh.refresh_token, c: now_secs() + jh.expires_in })
}
