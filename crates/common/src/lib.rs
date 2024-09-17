mod cbc;
pub use cbc::decrypt as cbc_decrypt;
pub use cbc::encrypt as cbc_encrypt;

mod oracle;
pub use oracle::{encryption_oracle, encryption_oracle2, has_repeats, EncryptionMethod};

pub mod kv;

pub fn xor(
    a: impl IntoIterator<Item = u8>,
    b: impl IntoIterator<Item = u8>,
) -> impl Iterator<Item = u8> {
    a.into_iter().zip(b).map(|(a, b)| a ^ b)
}

pub fn single_byte_xor(iter: impl IntoIterator<Item = u8>, val: u8) -> impl Iterator<Item = u8> {
    iter.into_iter().map(move |item| item ^ val)
}

pub fn repeating_xor(
    payload: impl IntoIterator<Item = u8>,
    key: impl IntoIterator<Item = u8>,
) -> impl Iterator<Item = u8> {
    let key: Vec<_> = key.into_iter().collect();
    let key = std::iter::repeat(key).flat_map(|i| i.into_iter());

    payload.into_iter().zip(key).map(|(a, b)| a ^ b)
}

pub fn score(phrase: impl IntoIterator<Item = u8>) -> usize {
    phrase.into_iter().fold(0, |acc, x| {
        let x = x as char;
        let is_frequently_used = x.is_ascii_alphanumeric() || x.is_ascii_whitespace();

        acc + if is_frequently_used { 1 } else { 0 }
    })
}

pub fn distance(a: impl IntoIterator<Item = u8>, b: impl IntoIterator<Item = u8>) -> u32 {
    let (mut a, mut b) = (a.into_iter(), b.into_iter());

    let mut count = 0;

    loop {
        match (a.next(), b.next()) {
            (Some(a), Some(b)) => count += (a ^ b).count_ones(),
            (Some(a), None) => count += a.count_ones(),
            (None, Some(b)) => count += b.count_ones(),
            (None, None) => break,
        }
    }

    count
}

pub fn pkcs7pad(input: &str, block_size: usize, padding: char) -> String {
    let mut output = input.to_string();
    let amount = block_size - input.len() % block_size;

    for _ in 0..amount {
        output.push(padding);
    }

    output
}

pub fn detect_pkcs7(input: impl AsRef<str>) -> bool {
    let input = input.as_ref().as_bytes();
    let count = match input.last() {
        Some(end) => end,
        None => return true,
    };

    if *count == 0 {
        return true;
    }

    for b in input.iter().rev().take(*count as usize) {
        if b != count {
            return false;
        }
    }

    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn xor_plain_works() {
        let a = hex::decode("1b37").unwrap();
        let b = hex::decode("4f2c").unwrap();
        let result: Vec<_> = xor(a, b).collect();
        let result = hex::encode(result);

        assert_eq!(result, "541b", "failed to xor properly");
    }

    #[test]
    fn xor_works() {
        let hex_iter =
            hex::decode("1b37373331363f78151b7f2b783431333d78397828372d363c78373e783a393b3736")
                .unwrap();
        let result: Vec<_> = single_byte_xor(hex_iter, 0x10).collect();
        let result = hex::encode(result);

        assert_eq!(
            result, "0b27272321262f68050b6f3b682421232d68296838273d262c68272e682a292b2726",
            "failed to xor properly"
        );
    }

    #[test]
    fn distance_works() {
        let a = "this is a test";
        let b = "wokka wokka!!!";

        let (a, b) = (a.as_bytes().iter().copied(), b.as_bytes().iter().copied());
        let distance = distance(a, b);

        assert_eq!(distance, 37);
    }

    #[test]
    fn padding_works() {
        let input = "YELLOW SUBMARINE";

        assert_eq!(
            pkcs7pad(input, 20, '\x04'),
            "YELLOW SUBMARINE\x04\x04\x04\x04"
        );
    }
}
