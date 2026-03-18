/// Developed BY XenithRed
/// Xbox Secure Token Service (XSTS)
///
/// Takes an XBL token and produces an XSTS token scoped to either:
///   - Java Edition  → relying party "rp://api.minecraftservices.com/"
///   - Bedrock       → relying party "https://multiplayer.minecraft.net/"
///
/// XSTS is the last Xbox-side auth step before hitting Minecraft's own APIs.
///
/// Error semantics: XSTS returns HTTP 401 with an XErr code on failure:
///   2148916233 → account has no Xbox Live profile (needs setup)
///   2148916235 → Xbox Live banned in user's region
///   2148916236 / 2148916237 → adult verification required
///   2148916238 → child account — parental consent needed
use crate::{
    error::{Err, R},
    token::{now_secs, XstsTok},
};
use reqwest::Client;
use serde::{Deserialize, Serialize};
use tracing::debug;

const A: &str = "https://xsts.auth.xboxlive.com/xsts/authorize";

// Relying parties
pub const RP_JAVA:    &str = "rp://api.minecraftservices.com/";
pub const RP_BEDROCK: &str = "https://multiplayer.minecraft.net/";

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
    #[serde(rename = "SandboxId")]
    a: &'x str,
    #[serde(rename = "UserTokens")]
    b: Vec<&'x str>,
}

#[derive(Deserialize)]
struct Resp {
    #[serde(rename = "Token")]
    a: String,
    #[serde(rename = "DisplayClaims")]
    b: Claims,
    #[serde(rename = "NotAfter")]
    c: String,
}

#[derive(Deserialize)]
struct Claims {
    xui: Vec<Xui>,
}

#[derive(Deserialize)]
struct Xui {
    uhs: String,
}

// 401 body when XSTS rejects the request
#[derive(Deserialize)]
struct XErr {
    #[serde(rename = "XErr")]
    code: u64,
}

// ── public API ────────────────────────────────────────────────────────────────

/// Exchange an XBL token for an XSTS token.
/// `rp` should be [`RP_JAVA`] or [`RP_BEDROCK`].
pub async fn auth(cx: &Client, xbl_tok: &str, rp: &str) -> R<XstsTok> {
    let zq = cx
        .post(A)
        .header("Accept", "application/json")
        .header("Content-Type", "application/json")
        .json(&Req {
            b: Props { a: "RETAIL", b: vec![xbl_tok] },
            c: rp,
            d: "JWT",
        })
        .send()
        .await?;

    // XSTS returns 401 on well-formed but rejected requests
    if zq.status() == reqwest::StatusCode::UNAUTHORIZED {
        let xe = zq.json::<XErr>().await.map(|x| x.code).unwrap_or(0);
        return Err(Err::Xsts(xe));
    }

    let jh = zq.error_for_status()?.json::<Resp>().await?;

    let uhs = jh
        .b
        .xui
        .into_iter()
        .next()
        .ok_or(Err::McAuth)?
        .uhs;

    let exp = crate::xbl::parse_iso_pub(&jh.c).unwrap_or(now_secs() + 86_400);

    debug!(rp, "xsts auth ok");
    Ok(XstsTok { a: jh.a, b: uhs, c: exp })
}
