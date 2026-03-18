use p256::{
    ecdh::EphemeralSecret,
    EncodedPoint, PublicKey,
};
use rand::rngs::OsRng;
use base64::{engine::general_purpose::STANDARD as B64, Engine};
use crate::error::{Err, R};
use super::aes::derive_key;

pub struct Ecdh {
    a: EphemeralSecret,
    b: EncodedPoint,
}

impl Ecdh {
    pub fn new() -> Self {
        let sk = EphemeralSecret::random(&mut OsRng);
        let pk = EncodedPoint::from(sk.public_key());
        Self { a: sk, b: pk }
    }

    pub fn pubkey_b64(&self) -> String {
        B64.encode(self.b.as_bytes())
    }

    pub fn pubkey_bytes(&self) -> &[u8] {
        self.b.as_bytes()
    }

    pub fn derive(&self, peer_b64: &str, salt: &[u8]) -> R<[u8; 32]> {
        let raw = B64.decode(peer_b64)
            .map_err(|e| Err::Crypto(e.to_string()))?;
        let peer = PublicKey::from_sec1_bytes(&raw)
            .map_err(|e| Err::Crypto(e.to_string()))?;
        let shared = self.a.diffie_hellman(&peer);
        Ok(derive_key(shared.raw_secret_bytes().as_slice(), salt))
    }
}
