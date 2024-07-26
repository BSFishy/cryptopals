use std::fs::read_to_string;

use ascii::AsciiString;

const KEY_COUNT: u8 = u8::MAX;

fn main() {
    let mut args = std::env::args();
    let filename = match args.nth(1) {
        Some(filename) => filename,
        None => panic!("you specify the file name to read from"),
    };

    let file_contents = read_to_string(filename).expect("failed to read file");
    let lines = file_contents.lines();

    // FIXME: clone here, which isn't super efficient. might be able to use size hint instead?
    let line_count = lines.clone().count();
    let mut solutions = Vec::with_capacity(KEY_COUNT as usize * line_count);

    for line in lines {
        let line = hex::decode(line).expect("failed to decode from hex");
        for key in 0..=KEY_COUNT {
            let result: Vec<_> = common::single_byte_xor(line.iter().copied(), key).collect();
            let score = common::score(result.iter().copied());
            let result = AsciiString::from_ascii(result)
                .map(|str| str.to_string())
                .unwrap_or("[failed to convert from ascii]".to_string());

            solutions.push((score, key, result));
        }
    }

    solutions.sort_by(|(a, _, _), (b, _, _)| b.cmp(a));

    println!("top 20 solutions:");
    for (score, key, string) in solutions.iter().take(20) {
        println!("\tsolution ({score:02}; 0x{key:02x}): {string:?}");
    }

    println!();

    let (score, key, string) = solutions.first().expect("no first solution??");
    println!(
        "Looks like the solution is {string:?} with a score of {score} and key of 0x{key:02x}"
    );
}
