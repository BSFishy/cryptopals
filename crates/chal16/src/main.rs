use std::panic;

use common::cbc_oracle::Oracle;

fn main() {
    let current_hook = panic::take_hook();
    panic::set_hook(Box::new(move |info| {
        eprintln!("\n\nthe command panicked. this maybe sometimes happens?? run again and see if it fails again\n\n");

        current_hook(info);
    }));

    let malicious_text = b";admin=true";
    let placeholder = vec![b'A'; malicious_text.len()];
    let pad: Vec<_> =
        common::xor(malicious_text.iter().copied(), placeholder.iter().copied()).collect();

    let oracle = Oracle::new();
    let data = oracle.encrypt(placeholder.iter().copied());
    let data = common::asymmetric_xor(data, {
        let mut data = Vec::with_capacity(pad.len() + 16);
        data.extend(vec![b'\x00'; 16]);
        data.extend(pad);
        data
    });

    let is_admin = oracle.is_admin(data);
    println!("is admin: {is_admin}");
}
