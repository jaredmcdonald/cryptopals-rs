use read_file::strings_from_filename;
use single_byte_xor::{most_englishy, single_byte_xor, EnglishyResult, EnglishyInput};
use ascii::bytes_to_ascii_string;
use hex::parse_hex;

pub fn run_04() {
    let lines = strings_from_filename("04.txt");
    let parsed_lines: Vec<Vec<u8>> = lines.iter().map(|s| parse_hex(&s)).collect();
    let mut xor_possibilities: Vec<EnglishyInput> = Vec::new();
    for line in parsed_lines {
        for xor_key in 0..255 {
            xor_possibilities.push(EnglishyInput {
                bytes: single_byte_xor(&line, xor_key), xor_key: xor_key
            });
        }
    }
    for result in most_englishy(&xor_possibilities) {
        let EnglishyResult { xor_key, bytes, test_result } = result;
        println!("Decoded: {}", bytes_to_ascii_string(&bytes));
        println!("Key: {:x}", xor_key);
        println!("K-S statistic: {}", test_result.statistic)
    }
}
