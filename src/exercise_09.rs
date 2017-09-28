use ascii::bytes_to_ascii_string;
use pkcs_7::pad;

pub fn run_09() {
    let input = "YELLOW SUBMARINE".as_bytes();
    let expected = "YELLOW SUBMARINE\x04\x04\x04\x04";
    let output = bytes_to_ascii_string(&pad(&input, 20));
    println!("exp: {:?}\ngot: {:?}\nmatches: {}", expected, output, output == expected);
}
