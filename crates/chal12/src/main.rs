use anyhow::{bail, Context, Result};
use ascii::AsciiString;
use common::has_repeats;

fn main() -> Result<()> {
    let block_size = detect_block_size()?;

    // times 3 to ensure at least 2 full blocks would be the same
    let example_encrypt = common::encryption_oracle2(vec![b'A'; block_size * 3]);
    if !has_repeats(&example_encrypt) {
        bail!("encryption is not ecb");
    }

    let message_len = detect_message_len(block_size)?;

    let mut msg = Vec::with_capacity(message_len);
    for _ in 0..message_len {
        let next_byte = detect_next_byte(&msg, block_size)?;
        msg.push(next_byte);
    }

    let string = AsciiString::from_ascii(msg).context("failed to convert to ascii string")?;
    println!("{string}");

    Ok(())
}

fn detect_block_size() -> Result<usize> {
    let max_size: usize = 100;
    let base_len = common::encryption_oracle2(vec![]).len();

    for i in 0..=max_size {
        let len = common::encryption_oracle2(vec![b'A'; i]).len();
        if len != base_len {
            return Ok(len - base_len);
        }
    }

    bail!("block size not within max size")
}

fn detect_message_len(block_size: usize) -> Result<usize> {
    let base_len = common::encryption_oracle2(vec![]).len();
    for i in 0..=(block_size + 1) {
        let len = common::encryption_oracle2(vec![b'A'; i]).len();
        if len > base_len {
            return Ok(base_len - i);
        }
    }

    bail!("failed to detect message length");
}

#[allow(clippy::ptr_arg)]
fn detect_next_byte(ref_block: &Vec<u8>, block_size: usize) -> Result<u8> {
    let i = ref_block.len();

    let guess_padding_len = block_size - i % block_size - 1;
    let mut guess_padding = vec![b'A'; guess_padding_len];
    guess_padding.append(&mut ref_block.clone());

    let actual_padding_len = guess_padding_len;
    let actual_padding = vec![b'A'; actual_padding_len];
    let actual = common::encryption_oracle2(actual_padding.clone());

    let block_to_check = i / block_size;

    for i in 0..=u8::MAX {
        let mut guess = guess_padding.clone();
        guess.push(i);

        let guess = common::encryption_oracle2(guess);

        let block_range = (block_to_check * block_size)..((block_to_check + 1) * block_size);
        let guess = &guess[block_range.clone()];
        let actual = &actual[block_range];
        if guess == actual {
            return Ok(i);
        }
    }

    bail!("failed to detect next byte")
}
