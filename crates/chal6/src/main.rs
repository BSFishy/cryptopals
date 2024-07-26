use std::fs::read_to_string;

use ascii::AsciiString;
use base64::{engine::general_purpose::STANDARD, Engine};

const KEYSIZE_MIN: usize = 2;
const KEYSIZE_MAX: usize = 40;
const KEYSIZE_BLOCKS: usize = 4;

fn main() {
    let mut args = std::env::args();
    let _cmd_name = args.next();
    let filename = args.next().expect("you must specify a filename");

    let file_contents = read_to_string(filename)
        .expect("failed to read file")
        .replace("\n", "");
    let file_contents = STANDARD
        .decode(file_contents)
        .expect("failed to base64 decode");

    // calculate keysizes and how viable they are as the final key size
    let mut keysizes = Vec::with_capacity(KEYSIZE_MAX - KEYSIZE_MIN + 1);
    for keysize in KEYSIZE_MIN..=KEYSIZE_MAX {
        let payload = file_contents.clone();

        // split the payload into `keysize` length blocks
        let mut blocks = Vec::with_capacity(KEYSIZE_BLOCKS);
        for i in 0..KEYSIZE_BLOCKS {
            let start = i * keysize;
            let end = (i + 1) * keysize;
            blocks.push(payload[start..end].iter().copied());
        }

        // calculate the distance between each block and sum it up
        let mut sum = 0f32;
        for window in blocks.windows(2) {
            let (a, b) = (window[0].clone(), window[1].clone());
            let distance = common::distance(a, b) as f32;
            let distance_norm = distance / keysize as f32;

            sum += distance_norm;
        }

        // store the average distance as the score of the keysize
        keysizes.push((keysize, sum / KEYSIZE_BLOCKS as f32));
    }

    // sort keysizes by score and only consider the first five from here on
    keysizes.sort_by(|(_, a), (_, b)| a.partial_cmp(b).expect("failed to compare"));
    let keysizes = &keysizes[0..5];

    // attempt to derive a key and decrypted payload for each keysize
    let mut cyphers = Vec::with_capacity(keysizes.len());
    for (keysize, _) in keysizes {
        let payload = file_contents.clone();
        let payload = payload.chunks(*keysize);
        let mut blocks = vec![Vec::new(); *keysize];

        // transpose blocks: first byte of each block is grouped, second byte of each block is
        // grouped, etc
        for sub in payload {
            for (i, x) in sub.iter().enumerate() {
                blocks[i].push(x);
            }
        }

        // calculate the keys for each block
        let mut keys = Vec::with_capacity(blocks[0].len());
        for block in blocks {
            // use sigle byte xor to attempt to find a key for this block then score it
            let mut scores = Vec::new();
            for i in 0..=255 {
                let x = common::single_byte_xor(block.clone().into_iter().copied(), i);
                let score = common::score(x);

                scores.push((score, i));
            }

            scores.sort_by(|(a, _), (b, _)| b.cmp(a));

            let (_, key) = scores.first().expect("no first score?");
            keys.push(*key);
        }

        // calculate a decrypted payload using the calculated key
        let payload = common::repeating_xor(file_contents.clone(), keys);
        let payload: Vec<_> = payload.collect();
        let payload = AsciiString::from_ascii(payload)
            .map(|str| str.to_string())
            .unwrap_or("[failed to decode ascii]".to_string());

        // score this key and push it to the rest of the payloads
        let score = common::score(payload.as_bytes().iter().copied());
        cyphers.push((score, payload));
    }

    // sort the decrypted payloads. this should bring the correct key/decrypted text to the top
    cyphers.sort_by(|(a, _), (b, _)| b.cmp(a));

    // we're done! (hopefully)
    let (_, solution) = cyphers.first().expect("no first cypher??");
    println!("{solution}");
}
