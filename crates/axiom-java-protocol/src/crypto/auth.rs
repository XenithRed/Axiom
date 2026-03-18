use sha1::{Digest, Sha1};
use reqwest::Client;
use serde::Deserialize;
use crate::error::{Err, R};

const A: &str = "https://sessionserver.mojang.com/session/minecraft/hasJoined";

#[derive(Debug, Clone, Deserialize)]
pub struct Profile {
    pub id:   String,
    pub name: String,
}

pub fn server_hash(server_id: &str, shared: &[u8], pub_key_der: &[u8]) -> String {
    let mut h = Sha1::new();
    h.update(server_id.as_bytes());
    h.update(shared);
    h.update(pub_key_der);
    let digest = h.finalize();
    hex_digest(&digest)
}

fn hex_digest(b: &[u8]) -> String {
    let neg = b[0] & 0x80 != 0;
    if neg {
        let mut n = b.to_vec();
        twos_complement(&mut n);
        format!("-{}", hex::encode(n).trim_start_matches('0'))
    } else {
        let s = hex::encode(b);
        s.trim_start_matches('0').to_string()
    }
}

fn twos_complement(b: &mut Vec<u8>) {
    let mut carry = true;
    for byte in b.iter_mut().rev() {
        let v = (!*byte) as u16 + carry as u16;
        *byte = v as u8;
        carry = v > 0xFF;
    }
}

pub async fn has_joined(
    cx:      &Client,
    name:    &str,
    hash:    &str,
    ip:      Option<&str>,
) -> R<Profile> {
    let mut url = format!("{A}?username={name}&serverId={hash}");
    if let Some(addr) = ip { url.push_str(&format!("&ip={addr}")); }

    let resp = cx.get(&url).send().await
        .map_err(|e| Err::Auth(e.to_string()))?;

    if resp.status() == reqwest::StatusCode::NO_CONTENT {
        return Err(Err::Auth("player has not joined".into()));
    }

    resp.error_for_status()
        .map_err(|e| Err::Auth(e.to_string()))?
        .json::<Profile>()
        .await
        .map_err(|e| Err::Auth(e.to_string()))
}
