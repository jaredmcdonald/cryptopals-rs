use read_file::strings_from_filename;
use english::{get_most_englishy, EnglishyResult, EnglishyInput};
use repeating_key_xor::repeating_key_xor;
use ascii::bytes_to_ascii_string;
use hex::parse_hex;

pub fn run_04() {
    let lines = strings_from_filename("04.txt");
    let parsed_lines: Vec<Vec<u8>> = lines.iter().map(|s| parse_hex(&s)).collect();
    let mut xor_possibilities: Vec<EnglishyInput> = Vec::new();
    for line in parsed_lines {
        for xor_key in 0..255 {
            let key = vec![xor_key];
            xor_possibilities.push(EnglishyInput {
                bytes: repeating_key_xor(&line, &key),
                xor_key: key
            });
        }
    }
    let EnglishyResult { xor_key, bytes, score } = get_most_englishy(&xor_possibilities);
    println!("Decoded: {}", bytes_to_ascii_string(&bytes));
    println!("Key: 0x{:02x}", xor_key[0]);
    println!("Score: {}", score);
}
