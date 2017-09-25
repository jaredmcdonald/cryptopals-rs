extern crate base64;
use base64::decode; // not gonna roll my own base64 decoder for now
use std::collections::HashMap;
use edit_distance::get_edit_distance;
use read_file::strings_from_filename;
use single_byte_xor::{guess_single_byte_xor, COMMON_CHARS};

fn transpose_bytes_by_keysize(bytes: &Vec<u8>, keysize: usize) -> Vec<Vec<u8>> {
    let mut transposed = Vec::new();
    for i in 0..keysize {
        transposed.push(Vec::new());
    }
    for i in 0..bytes.len() {
        transposed[i % keysize].push(bytes[i]);
    }
    transposed
}

fn normalized_keysize_score(bytes: &Vec<u8>, keysize: usize) -> f64 {
    // ack, working around my lack of knowledge around how to pass slices as arguments
    let mut first = Vec::new();
    let mut second = Vec::new();
    for i in 0..keysize * 2 {
        if i < keysize {
            first.push(bytes[i]);
        } else {
            second.push(bytes[i]);
        }
    }
    get_edit_distance(&first, &second) as f64 / keysize as f64
}

fn try_keysizes(bytes: &Vec<u8>) -> (usize, usize) {
    // hold onto the lowest few. stupid initial values, should really learn rust better
    let mut lowest = 99f64;
    let mut second_lowest = 99f64;
    let mut lowest_keysize = 99;
    let mut second_lowest_keysize = 99;
    for keysize in 2..41 {
        let score = normalized_keysize_score(bytes, keysize);
        if score < lowest {
            second_lowest = lowest;
            second_lowest_keysize = lowest_keysize;
            lowest = score;
            lowest_keysize = keysize;
        } else if score < second_lowest {
            second_lowest = score;
            second_lowest_keysize = keysize;
        }
    }
    (lowest_keysize, second_lowest_keysize)
}

fn decode_lines(lines: &Vec<String>) -> Vec<u8> {
    let mut bytes = Vec::new();
    for line in lines {
        // really need to figure out `map` :/
        bytes.extend(decode(line).unwrap());
    }
    bytes
}

pub fn run_06() {
    let lines = strings_from_filename("06.txt");
    let bytes = decode_lines(&lines);
    let keysizes = try_keysizes(&bytes);
    let transposed = transpose_bytes_by_keysize(&bytes, keysizes.0);
    for single_byte in transposed {
        println!("{}", guess_single_byte_xor(&single_byte, COMMON_CHARS[0]));
    }
}
