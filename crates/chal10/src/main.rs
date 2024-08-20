use ascii::AsciiString;
use base64::{engine::general_purpose::STANDARD, Engine};
use std::fs::read_to_string;

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

    let data = common::cbc_decrypt(&[0; 16], b"YELLOW SUBMARINE", file_contents, 4);
    match AsciiString::from_ascii(data) {
        Ok(ascii) => println!("{ascii}"),
        Err(err) => eprintln!("{err}"),
    }
}
