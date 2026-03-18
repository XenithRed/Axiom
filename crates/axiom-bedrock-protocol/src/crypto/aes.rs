use aes_gcm::{
    aead::{Aead, KeyInit, Payload},
    Aes256Gcm, Key, Nonce,
};
use crate::error::{Err, R};

pub const KEY_LEN:   usize = 32;
pub const NONCE_LEN: usize = 12;
pub const TAG_LEN:   usize = 16;

pub struct Cipher {
    a: Aes256Gcm,
    b: u64,
    c: u64,
}

impl Cipher {
    pub fn new(key: &[u8; KEY_LEN]) -> Self {
        let k = Key::<Aes256Gcm>::from_slice(key);
        Self { a: Aes256Gcm::new(k), b: 0, c: 0 }
    }

    pub fn encrypt(&mut self, plain: &[u8]) -> R<Vec<u8>> {
        let nonce = self.send_nonce();
        let ct = self.a
            .encrypt(&nonce, Payload { msg: plain, aad: b"" })
            .map_err(|e| Err::Crypto(e.to_string()))?;
        self.b += 1;
        Ok(ct)
    }

    pub fn decrypt(&mut self, cipher: &[u8]) -> R<Vec<u8>> {
        let nonce = self.recv_nonce();
        let pt = self.a
            .decrypt(&nonce, Payload { msg: cipher, aad: b"" })
            .map_err(|e| Err::Crypto(e.to_string()))?;
        self.c += 1;
        Ok(pt)
    }

    fn send_nonce(&self) -> aes_gcm::Nonce<aes_gcm::aead::generic_array::typenum::U12> {
        counter_nonce(self.b)
    }

    fn recv_nonce(&self) -> aes_gcm::Nonce<aes_gcm::aead::generic_array::typenum::U12> {
        counter_nonce(self.c)
    }
}

fn counter_nonce(n: u64) -> aes_gcm::Nonce<aes_gcm::aead::generic_array::typenum::U12> {
    let mut raw = [0u8; NONCE_LEN];
    raw[..8].copy_from_slice(&n.to_le_bytes());
    *Nonce::from_slice(&raw)
}

pub fn derive_key(shared: &[u8], salt: &[u8]) -> [u8; KEY_LEN] {
    use sha2::{Digest, Sha256};
    let mut h = Sha256::new();
    h.update(salt);
    h.update(shared);
    h.finalize().into()
}
