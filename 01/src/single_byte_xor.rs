use std::collections::HashMap;
use hex::parse_hex;
use ascii::bytes_to_ascii_string;

fn try_xor_by(bytes: &Vec<u8>, xor_by: u8) -> Vec<u8> {
    // can't figure out how to just do bytes.map and get the types to work out :/
    let mut xored = Vec::new();
    for b in bytes {
        xored.push(b ^ xor_by)
    }
    xored
}

fn byte_frequency(bytes: &Vec<u8>) -> Vec<&u8> {
    let mut frequency_table = HashMap::new();
    for byte in bytes {
        let counter = frequency_table.entry(byte).or_insert(0);
        *counter += 1;
    }
    let mut inverted = HashMap::new();
    let mut max_value = 0;
    for (key, value) in frequency_table {
        if value > max_value { max_value = value };
        let instances = inverted.entry(value).or_insert(Vec::new());
        instances.push(key);
    }
    let mut frequencies_ordered = Vec::new();
    for f in (0..max_value + 1).rev() {
        if let Some(bytes) = inverted.get(&f) {
            frequencies_ordered.extend(bytes);
        }
    }
    frequencies_ordered
}

fn guess_xor(frequencies: &Vec<&u8>, guess: &u8) -> u8 {
    frequencies[0] ^ guess
}

// http://reusablesec.blogspot.com/2009/05/character-frequency-analysis-info.html
pub const COMMON_CHARS: [u8; 6] = [0x20, 0x61, 0x45, 0x65, 0x54, 0x74];

pub fn guess_single_byte_xor_string(hex_input: &str, guess: u8) -> String {
    let bytes = parse_hex(hex_input);
    let frequencies = byte_frequency(&bytes);
    let xor_guess = guess_xor(&frequencies, &guess);
    let decoded = try_xor_by(&bytes, xor_guess);
    bytes_to_ascii_string(&decoded)
}

pub fn guess_single_byte_xor(bytes: &Vec<u8>, guess: u8) -> u8 {
    guess_xor(&byte_frequency(&bytes), &guess)
}
