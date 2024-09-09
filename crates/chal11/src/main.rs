use ascii::AsciiStr;
use common::{encryption_oracle, has_repeats, EncryptionMethod};

fn main() {
    let data = AsciiStr::from_ascii("XXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXX")
        .expect("failed to decode ascii")
        .as_bytes();
    let data = Vec::from(data);

    let (crypt, expected_method) = encryption_oracle(data);
    let method = if has_repeats(&crypt) {
        EncryptionMethod::Ecb
    } else {
        EncryptionMethod::Cbc
    };

    assert_eq!(expected_method, method);

    println!("encrypted {expected_method:?} and got it back");
}
