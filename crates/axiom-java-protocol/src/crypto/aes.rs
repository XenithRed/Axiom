use aes::Aes128;
use cfb_mode::{BufDecryptor, BufEncryptor};
use cfb_mode::cipher::{AsyncStreamCipher, KeyIvInit};
use crate::error::{Err, R};

type Enc128 = BufEncryptor<Aes128>;
type Dec128 = BufDecryptor<Aes128>;

pub struct AesCfb8 {
    a: Enc128,
    b: Dec128,
}

impl AesCfb8 {
    pub fn new(key: &[u8; 16], iv: &[u8; 16]) -> Self {
        Self {
            a: Enc128::new(key.into(), iv.into()),
            b: Dec128::new(key.into(), iv.into()),
        }
    }

    pub fn encrypt(&mut self, buf: &mut [u8]) {
        self.a.encrypt(buf);
    }

    pub fn decrypt(&mut self, buf: &mut [u8]) {
        self.b.decrypt(buf);
    }
}
