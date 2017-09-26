use read_file::strings_from_filename;
use single_byte_xor::{most_englishy, single_byte_xor};
use ascii::bytes_to_ascii_string;
use hex::parse_hex;

pub fn run_04() {
    let lines = strings_from_filename("04.txt");
    let parsed_lines: Vec<Vec<u8>> = lines.iter().map(|s| parse_hex(&s)).collect();
    let mut xor_possibilities: Vec<Vec<u8>> = Vec::new();
    for line in parsed_lines {
        for xor_value in 0..255 {
            xor_possibilities.push(single_byte_xor(&line, xor_value));
        }
    }
    for (bytes, score) in most_englishy(&xor_possibilities) {
        println!("{}: {}", score, bytes_to_ascii_string(&bytes));
    }
}
