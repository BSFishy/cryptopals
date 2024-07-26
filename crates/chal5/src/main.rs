use ascii::AsciiStr;

fn main() {
    println!(
        "{}",
        encrypt(
            "Burning 'em, if you ain't quick and nimble
I go crazy when I hear a cymbal",
            "ICE"
        )
    );
}

fn encrypt(payload: &str, key: &str) -> String {
    let payload = AsciiStr::from_ascii(payload).expect("failed to convert to ascii");
    let payload = payload.as_bytes();

    let key = AsciiStr::from_ascii(key).expect("failed to convert to ascii");
    let key = key.as_bytes();

    let cypher = common::repeating_xor(payload.iter().copied(), key.iter().copied());
    let cypher: Vec<_> = cypher.collect();

    hex::encode(cypher)
}
