use std::fs::read_to_string;

use ascii::AsciiString;
use base64::{engine::general_purpose::STANDARD, Engine};
use openssl::symm::{decrypt, Cipher};

fn main() {
    let mut args = std::env::args();
    let _command_name = args.next();
    let filename = match args.next() {
        Some(filename) => filename,
        None => panic!("you must specify a filename"),
    };

    let file_contents = read_to_string(filename)
        .expect("failed to read file")
        .replace("\n", "");
    let file_contents = STANDARD
        .decode(file_contents)
        .expect("failed to base64 decode");

    let cipher = Cipher::aes_128_ecb();
    let data = file_contents.as_slice();
    let key = b"YELLOW SUBMARINE";
    let cipher_text = decrypt(cipher, key, None, data).unwrap();
    let cipher_text = AsciiString::from_ascii(cipher_text).unwrap();

    println!("{cipher_text}");
}
