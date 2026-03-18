/// Developeed By XenithRed
/// Xbox Live Authentication
///
/// Exchanges an MSA access_token for an XBL token.
/// XBL uses a JSON-over-HTTPS RPS protocol, not OAuth.
///
/// POST https://user.auth.xboxlive.com/user/authenticate
///   body: { "Properties": { "AuthMethod":"RPS", "SiteName":"user.auth.xboxlive.com", "RpsTicket":"d=<msa_token>" },
///           "RelyingParty": "http://auth.xboxlive.com", "TokenType": "JWT" }
use crate::{
    error::{Err, R},
    token::{now_secs, XblTok},
};
use reqwest::Client;
use serde::{Deserialize, Serialize};
use tracing::debug;

const A: &str = "https://user.auth.xboxlive.com/user/authenticate";

// ── wire types ────────────────────────────────────────────────────────────────

#[derive(Serialize)]
struct Req<'x> {
    #[serde(rename = "Properties")]
    b: Props<'x>,
    #[serde(rename = "RelyingParty")]
    c: &'x str,
    #[serde(rename = "TokenType")]
    d: &'x str,
}

#[derive(Serialize)]
struct Props<'x> {
    #[serde(rename = "AuthMethod")]
    a: &'x str,
    #[serde(rename = "SiteName")]
    b: &'x str,
    #[serde(rename = "RpsTicket")]
    c: String,
}

#[derive(Deserialize)]
struct Resp {
    #[serde(rename = "Token")]
    a: String,
    #[serde(rename = "DisplayClaims")]
    b: Claims,
    #[serde(rename = "NotAfter")]
    c: String,   // ISO-8601 e.g. "2024-01-01T00:00:00.0000000Z"
}

#[derive(Deserialize)]
struct Claims {
    xui: Vec<Xui>,
}

#[derive(Deserialize)]
struct Xui {
    uhs: String,
}

// ── public API ────────────────────────────────────────────────────────────────

/// Exchange MSA access token for an XBL token.
pub async fn auth(cx: &Client, msa_at: &str) -> R<XblTok> {
    let jh = cx
        .post(A)
        .header("Accept", "application/json")
        .header("Content-Type", "application/json")
        .json(&Req {
            b: Props {
                a: "RPS",
                b: "user.auth.xboxlive.com",
                c: format!("d={msa_at}"),
            },
            c: "http://auth.xboxlive.com",
            d: "JWT",
        })
        .send()
        .await?
        .error_for_status()
        .map_err(|e| Err::Xbl(e.to_string()))?
        .json::<Resp>()
        .await?;

    let uhs = jh
        .b
        .xui
        .into_iter()
        .next()
        .ok_or_else(|| Err::Xbl("missing uhs claim".into()))?
        .uhs;

    // ISO-8601 → unix seconds with a simple parse
    // e.g. "2025-06-01T12:00:00.0000000Z"
    let exp = parse_iso(&jh.c).unwrap_or(now_secs() + 86_400);

    debug!(uhs = %uhs, "xbl auth ok");
    Ok(XblTok { a: jh.a, b: uhs, c: exp })
}

// ── helpers ───────────────────────────────────────────────────────────────────

pub(crate) fn parse_iso_pub(s: &str) -> Option<u64> { parse_iso(s) }

fn parse_iso(s: &str) -> Option<u64> {
    // Truncate at '.' so we handle both with and without sub-second part
    let base = s.split('.').next()?;           // "2025-06-01T12:00:00"
    let base = base.trim_end_matches('Z');
    let (date, time) = base.split_once('T')?;
    let mut dp = date.splitn(3, '-');
    let yr: i64 = dp.next()?.parse().ok()?;
    let mo: i64 = dp.next()?.parse().ok()?;
    let dy: i64 = dp.next()?.parse().ok()?;
    let mut tp = time.splitn(3, ':');
    let hh: i64 = tp.next()?.parse().ok()?;
    let mm: i64 = tp.next()?.parse().ok()?;
    let ss: i64 = tp.next()?.parse().ok()?;

    // Rough unix second calculation (good enough for expiry checks)
    let days_epoch = days_since_epoch(yr, mo, dy);
    let unix = days_epoch * 86_400 + hh * 3_600 + mm * 60 + ss;
    Some(unix as u64)
}

fn days_since_epoch(yr: i64, mo: i64, dy: i64) -> i64 {
    // Rata Die formula → shift to Unix epoch (1970-01-01 = day 719_163)
    let (y, m) = if mo <= 2 { (yr - 1, mo + 9) } else { (yr, mo - 3) };
    let c   = y / 100;
    let rd  = 365 * y + y / 4 - c + c / 4 + (153 * m + 2) / 5 + dy - 1;
    rd - 719_162
}
