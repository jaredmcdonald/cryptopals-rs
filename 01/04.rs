mod single_byte_xor;
mod hex;

use std::io::BufReader;
use std::io::BufRead;
use std::fs::File;
use std::path::Path;
use single_byte_xor::{guess_single_byte_xor, COMMON_CHARS};

fn strings_from_file() -> Vec<String> {
    let mut strings = Vec::new();
    if let Ok(f) = File::open("04.txt") {
        let file = BufReader::new(&f);
        for line in file.lines() {
            strings.push(line.unwrap());
        }
    }
    strings
}

fn main() {
    for hex_string in strings_from_file() {
        println!("{}", guess_single_byte_xor(&hex_string, COMMON_CHARS[0]));
    }
}
