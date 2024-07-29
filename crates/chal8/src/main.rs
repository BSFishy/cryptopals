use std::{collections::BTreeSet, fs::read_to_string};

fn main() {
    let mut args = std::env::args();
    let _command_name = args.next();
    let filename = match args.next() {
        Some(filename) => filename,
        None => panic!("you must specify a filename"),
    };

    let file_contents = read_to_string(filename).expect("failed to read file");
    for line in file_contents.lines() {
        if has_repeats(line) {
            let data: Vec<_> = hex::decode(line)
                .unwrap()
                .iter()
                .map(|byte| format!("{byte:02x}"))
                .collect();
            let data = data.join(", ");

            println!("has repeats: [{data}]");
        }
    }
}

fn has_repeats(line: &str) -> bool {
    const BLOCK_SIZE: usize = 16;
    let data = hex::decode(line).expect("failed to decode hex");
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
