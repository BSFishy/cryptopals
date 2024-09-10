fn main() {
    let data = "t=test&t2=test 2";
    let mut output = common::kv::parse(data.as_bytes()).expect("failed to parse");

    println!("{output:?}");

    output.insert("email".to_string(), "foo@bar.com&role=admin".to_string());

    let encode = common::kv::encode(&output);
    println!("{}", encode);
    println!("{:?}", common::kv::parse(encode.as_bytes()));
}
