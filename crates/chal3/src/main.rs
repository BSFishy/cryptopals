use ascii::AsciiString;

fn main() {
    let mut solutions = Vec::with_capacity(256);

    for i in 0..=255 {
        let data =
            hex::decode("1b37373331363f78151b7f2b783431333d78397828372d363c78373e783a393b3736")
                .unwrap();

        let result: Vec<_> = common::single_byte_xor(data.iter().copied(), i).collect();
        let score = common::score(result.iter().copied());
        let result = AsciiString::from_ascii(result)
            .map(|str| str.to_string())
            .unwrap_or("[failed to convert to ascii]".to_string());

        solutions.push((score, i, result));
    }

    solutions.sort_by(|(a, _, _), (b, _, _)| b.cmp(a));

    println!("top 20 solutions:");
    for (score, key, value) in solutions.iter().take(20) {
        println!("\tsolution ({score:02}; 0x{key:02x}): {value:?}");
    }

    println!();

    let (score, key, value) = solutions.first().expect("no first item?");
    println!(
        "Looks like our winner is \"{value}\" with a score of {score}, using the key 0x{key:02x}"
    );
}
