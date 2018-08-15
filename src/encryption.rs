use chacha;
use chacha::KeyStream;
use rand::{thread_rng, Rng};
use sha3::{Digest, Sha3_256};

pub fn random_fill_25(buf: &mut [u8]) {
    let mut res = [0u8; 25];
    thread_rng().fill(&mut res);
    buf.copy_from_slice(&res);
}

pub fn is_tampered_16(buf: &[u8]) -> bool {
    let mut tampered = false;
    for i in 0..8 {
        if buf[i] != buf[i + 8] {
            tampered = true;
        }
    }
    tampered
}

pub struct ChaCha {
    internal: Option<chacha::ChaCha>,
}

impl ChaCha {
    pub fn new(should_be_encrypted: bool, password: &str, nonce: &[u8; 24]) -> ChaCha {
        if should_be_encrypted {
            let secret_key = {
                let mut hasher = Sha3_256::default();
                hasher.input(password.as_bytes());
                let mut result = [0u8; 32];
                result.copy_from_slice(hasher.result().as_slice());
                result
            };
            ChaCha {
                internal: Some(chacha::ChaCha::new_xchacha20(&secret_key, nonce)),
            }
        } else {
            ChaCha { internal: None }
        }
    }
    pub fn xor(&mut self, dest: &mut [u8]) {
        if let Some(ref mut chacha) = self.internal {
            chacha.xor_read(dest).is_ok();
        }
    }
}
