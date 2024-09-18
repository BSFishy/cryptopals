use ascii::AsciiString;
use openssl::symm::{decrypt, encrypt, Cipher};
use percent_encoding::{percent_encode, AsciiSet, CONTROLS};

const FRAGMENT: &AsciiSet = &CONTROLS.add(b';').add(b'=');

pub struct Oracle {
    key: [u8; 16],
    iv: [u8; 16],
}

impl Oracle {
    pub fn new() -> Self {
        Self {
            key: rand::random(),
            iv: rand::random(),
        }
    }

    pub fn encrypt(&self, input: impl IntoIterator<Item = u8>) -> Vec<u8> {
        let input = input.into_iter();
        let input: Vec<u8> = input.collect();

        let data = {
            let mut data = Vec::new();
            data.extend(b"comment1=cooking%20MCs;userdata=");
            data.extend(percent_encode(input.as_slice(), FRAGMENT).flat_map(|s| s.as_bytes()));
            data.extend(b";comment2=%20like%20a%20pound%20of%20bacon");

            data
        };

        let cipher = Cipher::aes_128_cbc();
        encrypt(cipher, &self.key, Some(&self.iv), data.as_slice()).expect("failed to encrypt")
    }

    pub fn decrypt(&self, data: impl IntoIterator<Item = u8>) -> Vec<u8> {
        let data = data.into_iter();
        let data: Vec<u8> = data.collect();

        let cipher = Cipher::aes_128_cbc();
        decrypt(cipher, &self.key, Some(&self.iv), data.as_slice()).expect("failed to decrypt")
    }

    pub fn is_admin(&self, data: impl IntoIterator<Item = u8>) -> bool {
        let data = self.decrypt(data);
        // SAFETY: this operation is not safe.
        let data = unsafe { AsciiString::from_ascii_unchecked(data) };
        let data = data.as_str();
        let mut data = data
            .split(';')
            .map(|e| e.split_once('=').expect("malformed kv pair"));

        data.find_map(|(k, v)| {
            if k == "admin" {
                Some(v == "true")
            } else {
                None
            }
        })
        .unwrap_or(false)
    }
}

impl Default for Oracle {
    fn default() -> Self {
        Self::new()
    }
}
