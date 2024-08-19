fn main() {
    println!(
        "padded string: {:?}",
        common::pkcs7pad("YELLOW SUBMARINE", 20, '\x04')
    )
}
