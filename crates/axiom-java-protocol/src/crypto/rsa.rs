use rsa::{RsaPrivateKey, RsaPublicKey, pkcs8::DecodePublicKey};
use rsa::pkcs1v15::Pkcs1v15Encrypt;
use rand::rngs::OsRng;
use crate::error::{Err, R};

pub const KEY_BITS: usize = 1024;

pub struct ServerKey {
    a: RsaPrivateKey,
    b: RsaPublicKey,
    c: Vec<u8>,
}

impl ServerKey {
    pub fn generate() -> R<Self> {
        let sk = RsaPrivateKey::new(&mut OsRng, KEY_BITS)
            .map_err(|e| Err::Rsa(e.to_string()))?;
        let pk = RsaPublicKey::from(&sk);
        let der = rsa::pkcs8::EncodePublicKey::to_public_key_der(&pk)
            .map_err(|e| Err::Rsa(e.to_string()))?
            .into_vec();
        Ok(Self { a: sk, b: pk, c: der })
    }

    pub fn public_der(&self) -> &[u8] { &self.c }

    pub fn decrypt(&self, cipher: &[u8]) -> R<Vec<u8>> {
        self.a.decrypt(Pkcs1v15Encrypt, cipher)
            .map_err(|e| Err::Rsa(e.to_string()))
    }
}

pub fn verify_token(key: &ServerKey, encrypted: &[u8], expected: &[u8]) -> R<bool> {
    let plain = key.decrypt(encrypted)?;
    Ok(plain == expected)
}

pub fn gen_verify_token() -> [u8; 4] {
    use rand::RngCore;
    let mut t = [0u8; 4];
    OsRng.fill_bytes(&mut t);
    t
}
