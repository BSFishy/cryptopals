use aes::{
    cipher::{BlockDecryptMut, BlockEncryptMut, KeyInit},
    Block,
};

type Aes128EcbEnc = ecb::Encryptor<aes::Aes128>;
type Aes128EcbDec = ecb::Decryptor<aes::Aes128>;

fn pad(data: Vec<u8>) -> (Vec<u8>, usize) {
    let block_size = 16;
    let padding = 4;

    let pad_size = block_size - data.len() % block_size;
    if pad_size == block_size {
        return (data, 0);
    }

    let mut out = data.clone();
    out.resize(data.len() + pad_size, padding);

    (out, pad_size)
}

pub fn encrypt(iv: &[u8], key: &[u8], data: Vec<u8>) -> (Vec<u8>, usize) {
    assert!(iv.len() == 16);
    assert!(key.len() == 16);

    let (mut data, pad_size) = pad(data);
    let mut out = Vec::with_capacity(data.len());
    let mut previous_block = Vec::from(iv);
    let mut cypher = Aes128EcbEnc::new(Block::from_slice(key));

    for block in data.chunks_mut(16) {
        // xor the block with the previous block
        let block = Block::from_mut_slice(block);
        let block = crate::xor(block.iter().copied(), previous_block.iter().copied());

        // ecb the block
        let mut block = Block::from_iter(block);
        cypher.encrypt_block_mut(&mut block);

        // add the block to the output
        let block = block.to_vec();
        out.append(&mut block.clone());

        previous_block = block;
    }

    (out, pad_size)
}

pub fn decrypt(iv: &[u8], key: &[u8], mut data: Vec<u8>, pad_size: usize) -> Vec<u8> {
    assert!(iv.len() == 16);
    assert!(key.len() == 16);
    assert!(data.len() % 16 == 0);

    let mut out = Vec::with_capacity(data.len());
    let mut previous_block = Vec::from(iv);
    let mut cypher = Aes128EcbDec::new(Block::from_slice(key));

    for block in data.chunks_mut(16) {
        // ecb decrypt the block
        let block = Block::from_mut_slice(block);
        let tmp = block.iter().copied().collect();
        cypher.decrypt_block_mut(block);

        // xor the block with the previous block
        let block = crate::xor(block.iter().copied(), previous_block.iter().copied());
        let block: Vec<_> = block.into_iter().collect();

        // add the block to the output
        out.append(&mut block.clone());

        // the previous block needs to be the encrypted block
        previous_block = tmp;
    }

    for i in ((out.len() - pad_size)..out.len()).rev() {
        out.remove(i);
    }

    out
}
