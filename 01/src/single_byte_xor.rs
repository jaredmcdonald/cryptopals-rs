use kolmogorov_smirnov::test::{test, TestResult};

use std::collections::HashMap;
use hex::parse_hex;
use ascii::ascii_to_bytes;
use ascii::bytes_to_ascii_string;

const ENGLISH_TRAINING_TEXT: &str = "The Kolmogorov-Smirnov test is a hypothesis test procedure for determining if two samples of data are from the same distribution. The test is non-parametric and entirely agnostic to what this distribution actually is. The fact that we never have to know the distribution the samples come from is incredibly useful, especially in software and operations where the distributions are hard to express and difficult to calculate with.";

pub fn most_englishy(attempts: &Vec<Vec<u8>>) -> (Vec<u8>, f64) {
    // TODO: need to change the K-S inputs to frequency probabilities
    // right now this makes no sense, treating the ASCII byte labels as significant
    let training_text_parsed = ascii_to_bytes(ENGLISH_TRAINING_TEXT);
    let mut highest_statistic = 0f64;
    let mut highest_scoring: Vec<u8> = Vec::new();
    for attempt in attempts {
        let result = test(&training_text_parsed, attempt, 0.95);
        if result.statistic > highest_statistic {
            highest_statistic = result.statistic;
            highest_scoring = attempt.to_vec(); // the compiler made me do it!
        }
    }
    (highest_scoring, highest_statistic)
}

pub fn single_byte_xor(bytes: &Vec<u8>, xor_byte: u8) -> Vec<u8> {
    bytes.iter().map(|byte| byte ^ xor_byte).collect()
}

// fn try_xor_by(bytes: &Vec<u8>, xor_by: u8) -> Vec<u8> {
//     // can't figure out how to just do bytes.map and get the types to work out :/
//     let mut xored = Vec::new();
//     for b in bytes {
//         xored.push(b ^ xor_by)
//     }
//     xored
// }
//
// fn byte_frequency(bytes: &Vec<u8>) -> Vec<&u8> {
//     let mut frequency_table = HashMap::new();
//     for byte in bytes {
//         let counter = frequency_table.entry(byte).or_insert(0);
//         *counter += 1;
//     }
//     let mut inverted = HashMap::new();
//     let mut max_value = 0;
//     for (key, value) in frequency_table {
//         if value > max_value { max_value = value };
//         let instances = inverted.entry(value).or_insert(Vec::new());
//         instances.push(key);
//     }
//     let mut frequencies_ordered = Vec::new();
//     for f in (0..max_value + 1).rev() {
//         if let Some(bytes) = inverted.get(&f) {
//             frequencies_ordered.extend(bytes);
//         }
//     }
//     frequencies_ordered
// }
//
// fn guess_xor(frequencies: &Vec<&u8>, guess: &u8) -> u8 {
//     frequencies[0] ^ guess
// }
//
// http://reusablesec.blogspot.com/2009/05/character-frequency-analysis-info.html
pub const COMMON_CHARS: [u8; 6] = [0x20, 0x61, 0x45, 0x65, 0x54, 0x74];

// pub fn guess_single_byte_xor_string(hex_input: &str, guess: u8) -> String {
//     let bytes = parse_hex(hex_input);
//     let frequencies = byte_frequency(&bytes);
//     let xor_guess = guess_xor(&frequencies, &guess);
//     let decoded = try_xor_by(&bytes, xor_guess);
//     bytes_to_ascii_string(&decoded)
// }
//
// pub fn guess_single_byte_xor(bytes: &Vec<u8>, guess: u8) -> u8 {
//     guess_xor(&byte_frequency(&bytes), &guess)
// }
