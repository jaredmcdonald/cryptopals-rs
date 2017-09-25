use read_file::strings_from_filename;
use single_byte_xor::{guess_single_byte_xor, COMMON_CHARS};
use std::env;

pub fn run_04() {
    for hex_string in strings_from_filename("04.txt") {
        println!("{}", guess_single_byte_xor(&hex_string, COMMON_CHARS[0]));
    }
}
