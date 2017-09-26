extern crate base64; // not gonna roll my own base64 decoder for now

use base64::decode;
use std::cmp::{Ord, Ordering, PartialEq};

use edit_distance::get_edit_distance;
use read_file::strings_from_filename;
use repeating_key_xor::repeating_key_xor;
use ascii::bytes_to_ascii_string;
use english::{most_englishy, EnglishyInput};

fn transpose_bytes_by_keysize(bytes: &Vec<u8>, keysize: usize) -> Vec<Vec<u8>> {
    let mut transposed = Vec::new();
    for _ in 0..keysize {
        transposed.push(Vec::new());
    }
    for i in 0..bytes.len() {
        transposed[i % keysize].push(bytes[i]);
    }
    transposed
}

fn normalized_keysize_score(bytes: &Vec<u8>, keysize: usize) -> f64 {
    let first = bytes.get(0..keysize).unwrap().to_vec();
    let second = bytes.get(keysize..keysize * 2).unwrap().to_vec();
    let third = bytes.get(keysize * 2..keysize * 3).unwrap().to_vec();
    let fourth = bytes.get(keysize * 3..keysize * 4).unwrap().to_vec();
    let average_distance = (get_edit_distance(&first, &second) +
                            get_edit_distance(&first, &third) +
                            get_edit_distance(&first, &fourth)) as f64 / 3f64;
    average_distance / (keysize as f64)
}

struct KeysizeScore {
    pub size: usize,
    score: f64,
}

impl Ord for KeysizeScore {
    fn cmp(&self, other: &KeysizeScore) -> Ordering {
        self.score.partial_cmp(&other.score).unwrap_or(Ordering::Less)
    }
}

impl PartialOrd for KeysizeScore {
    fn partial_cmp(&self, other: &KeysizeScore) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for KeysizeScore {
    fn eq(&self, other: &KeysizeScore) -> bool {
        self.size == other.size && self.score == other.score
    }
}
impl Eq for KeysizeScore {}

fn try_keysizes(bytes: &Vec<u8>) -> Vec<usize> {
    let mut keysize_scores = Vec::new();
    for keysize in 2..41 {
        let keysize_score = KeysizeScore {
            score: normalized_keysize_score(bytes, keysize),
            size: keysize,
        };
        match keysize_scores.binary_search(&keysize_score) {
            Ok(pos) => keysize_scores.insert(pos, keysize_score),
            Err(pos) => keysize_scores.insert(pos, keysize_score),
        }
    }
    keysize_scores.iter().map(|keysize| keysize.size).collect()
}

fn flatten_lines(byte_lines: &Vec<Vec<u8>>) -> Vec<u8> {
    let mut bytes = Vec::new();
    for byte in byte_lines {
        bytes.extend(byte);
    }
    bytes
}

fn decode_lines(lines: &Vec<String>) -> Vec<Vec<u8>> {
    let mut bytes = Vec::new();
    for line in lines {
        // really need to figure out `map`, `reduce`, etc :/
        bytes.push(decode(line).unwrap());
    }
    bytes
}

pub fn run_06() {
    let line_strings = strings_from_filename("06.txt");
    let lines = decode_lines(&line_strings);
    let flattened_bytes = flatten_lines(&lines);
    let keysizes = try_keysizes(&flattened_bytes).get(0..10).unwrap().to_vec(); // try the first ten :/

    let mut possible_keys = Vec::new();

    'keysize: for keysize in keysizes {
        let transposed = transpose_bytes_by_keysize(&flattened_bytes, keysize);
        let mut repeating_key = Vec::new();
        for transposed_bytes in transposed {
            let mut possibilities = Vec::new();
            for xor_key in 0..255 {
                let key = vec![xor_key];
                possibilities.push(EnglishyInput {
                    xor_key: key.to_vec(),
                    bytes: repeating_key_xor(&transposed_bytes, &key),
                });
            }
            if let Some(result) = most_englishy(&possibilities) {
                repeating_key.push(result.xor_key[0]);
            } else {
                // it's probably not this keysize if we couldn't get any to pass,
                // so skip the rest by continuing the outer loop
                continue 'keysize;
            }
        }
        possible_keys.push(repeating_key);
    }

    for key in possible_keys {
        println!("Key:{:?}\n\nDecryption attempt:\n{}", key, bytes_to_ascii_string(&repeating_key_xor(&flattened_bytes, &key)));
    }
}
