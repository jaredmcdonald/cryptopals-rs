use ascii::bytes_to_ascii_string;

fn pkcs_7(bytes: &[u8], block_size: u8) -> Vec<u8> {
    let bytes_len = bytes.len() as u8;
    assert!(bytes_len <= block_size,
        "pkcs_7: block size must be greater than or equal to the size of the input");
    let padding = block_size % bytes_len;
    let mut padded = Vec::new();
    padded.extend(bytes);
    padded.extend(vec![padding; padding as usize]);
    padded
}

pub fn run_09() {
    let input = "YELLOW SUBMARINE".as_bytes();
    let expected = "YELLOW SUBMARINE\x04\x04\x04\x04";
    let output = bytes_to_ascii_string(&pkcs_7(&input, 20));
    println!("exp: {:?}\ngot: {:?}\nmatches: {}", expected, output, output == expected);
}
