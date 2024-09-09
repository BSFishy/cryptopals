use std::{collections::BTreeSet, sync::LazyLock};

use base64::{engine::general_purpose::STANDARD, Engine};
use openssl::symm::{encrypt, Cipher};
use rand::{rngs::ThreadRng, Rng};

fn rng_vec(rng: &mut ThreadRng) -> Vec<u8> {
    let len: usize = rng.gen_range(5..=10);
    let mut out = Vec::with_capacity(len);

    for _ in 0..len {
        out.push(rng.gen());
    }

    out
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EncryptionMethod {
    Ecb,
    Cbc,
}

pub fn encryption_oracle(mut input: Vec<u8>) -> (Vec<u8>, EncryptionMethod) {
    let mut rng = rand::thread_rng();
    let key: [u8; 16] = rng.gen();
    let prefix = rng_vec(&mut rng);
    let mut suffix = rng_vec(&mut rng);

    let data = {
        let mut out = prefix;
        out.append(&mut input);
        out.append(&mut suffix);
        out
    };

    if rng.gen() {
        let cipher = Cipher::aes_128_ecb();

        (
            encrypt(cipher, &key, None, data.as_slice()).expect("failed to encrypt"),
            EncryptionMethod::Ecb,
        )
    } else {
        let iv: [u8; 16] = rng.gen();

        let (out, _) = crate::cbc_encrypt(&iv, &key, data);
        (out, EncryptionMethod::Cbc)
    }
}

static COMMON_KEY: LazyLock<[u8; 16]> = LazyLock::new(rand::random);

pub fn encryption_oracle2(mut input: Vec<u8>) -> Vec<u8> {
    let mut data = STANDARD.decode("Um9sbGluJyBpbiBteSA1LjAKV2l0aCBteSByYWctdG9wIGRvd24gc28gbXkgaGFpciBjYW4gYmxvdwpUaGUgZ2lybGllcyBvbiBzdGFuZGJ5IHdhdmluZyBqdXN0IHRvIHNheSBoaQpEaWQgeW91IHN0b3A/IE5vLCBJIGp1c3QgZHJvdmUgYnkK").expect("failed to decode oracle prefix");
    // data.append(&mut input);
    input.append(&mut data);

    let cipher = Cipher::aes_128_ecb();
    encrypt(cipher, &*COMMON_KEY, None, input.as_slice()).expect("failed to encrypt")
}

pub fn has_repeats(data: &[u8]) -> bool {
    const BLOCK_SIZE: usize = 16;
    let num_blocks = data.len() / BLOCK_SIZE;

    let blocks = {
        let mut blocks = BTreeSet::new();
        for i in 0..num_blocks {
            let mut block = [0; BLOCK_SIZE];
            for (i, byte) in data[i * BLOCK_SIZE..(i + 1) * BLOCK_SIZE]
                .iter()
                .enumerate()
            {
                block[i] = *byte;
            }
            blocks.insert(block);
        }
        blocks
    };

    blocks.len() != num_blocks
}
