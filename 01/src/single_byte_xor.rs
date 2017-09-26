use kolmogorov_smirnov::test_f64;

use std::collections::HashMap;
use hex::parse_hex;
use ascii::ascii_to_bytes;
use ascii::bytes_to_ascii_string;

const ENGLISH_TRAINING_TEXT: &str = "The Kolmogorov-Smirnov test is a hypothesis test procedure for determining if two samples of data are from the same distribution. The test is non-parametric and entirely agnostic to what this distribution actually is. The fact that we never have to know the distribution the samples come from is incredibly useful, especially in software and operations where the distributions are hard to express and difficult to calculate with.";

pub fn most_englishy(attempts: &Vec<Vec<u8>>) -> Vec<(&Vec<u8>, f64)> {
    // TODO: need to change the K-S inputs to frequency probabilities
    // right now this makes no sense, treating the ASCII byte labels as significant
    // let training_text_parsed = byte_frequency(&ascii_to_bytes(ENGLISH_TRAINING_TEXT));
    // http://reusablesec.blogspot.com/2009/05/character-frequency-analysis-info.html
    let training_text_parsed = [
        7.52766, 7.0925, 5.17, 4.96032, 4.69732, 4.61079, 4.56899, 4.35053, 3.87388, 3.77728,
        3.12312, 2.99913, 2.76401, 2.74381, 2.57276, 2.45578, 2.43339, 2.41319, 2.29145, 2.10191,
        1.96828, 1.94265, 1.88577, 1.85331, 1.79558, 1.75647, 1.66225, 1.621, 1.52483, 1.2476,
        1.24492, 0.836677, 0.833626, 0.632558, 0.573305, 0.346119, 0.130466, 0.108132, 0.0970865,
        0.08476, 0.0806715, 0.0801223, 0.0782306, 0.0775594, 0.0748134, 0.073715, 0.0729217,
        0.070908, 0.0698096, 0.0660872, 0.0544319, 0.0497332, 0.0460719, 0.0417393, 0.0363083,
        0.0350268, 0.0320367, 0.0316706, 0.0306942, 0.0255073, 0.0241648, 0.0238597, 0.0235546,
        0.0197712, 0.0170252, 0.0147064, 0.0142182, 0.0122655, 0.00970255, 0.00854313, 0.00323418,
        0.00311214, 0.00231885, 0.00207476, 0.00207476, 0.00195272, 0.00189169, 0.00170863,
        0.00152556, 0.00140351, 0.00134249, 0.00115942, 0.00115942, 0.00115942, 0.0010984, 0.0010984
    ];
    // let mut highest_statistic = 0f64;
    // let mut highest_scoring: Vec<u8> = Vec::new();

    let mut passing = Vec::new();

    for attempt in attempts {
        let result = test_f64(&training_text_parsed, &byte_frequency(&attempt), 0.5);

        if !result.is_rejected {
            passing.push((attempt, result.statistic));
        }
    }
    passing
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
fn byte_frequency(bytes: &Vec<u8>) -> Vec<f64> {
    let mut frequency_table = HashMap::new();
    for byte in bytes {
        let counter = frequency_table.entry(byte).or_insert(0f64);
        *counter += 1f64;
    }
    let mut probabilities = Vec::new();
    let len = bytes.len() as f64;
    for (_, value) in frequency_table {
        probabilities.push(value / len);
    }
    probabilities
}
//
// fn guess_xor(frequencies: &Vec<&u8>, guess: &u8) -> u8 {
//     frequencies[0] ^ guess
// }
//
// http://reusablesec.blogspot.com/2009/05/character-frequency-analysis-info.html
// pub const COMMON_CHARS: [u8; 6] = [0x20, 0x61, 0x45, 0x65, 0x54, 0x74];

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
