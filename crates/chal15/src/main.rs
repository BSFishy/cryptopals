use common::detect_pkcs7;

macro_rules! pretty_print {
    ($e:expr) => {
        println!("{} ({})", $e, stringify!($e))
    };
}

fn main() {
    pretty_print!(detect_pkcs7("ICE ICE BABY\x04\x04\x04\x04"));
    pretty_print!(detect_pkcs7("ICE ICE BABY\x05\x05\x05\x05"));
    pretty_print!(detect_pkcs7("ICE ICE BABY\x01\x02\x03\x04"));
}
