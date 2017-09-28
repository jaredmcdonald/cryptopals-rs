#[derive(Debug)]
pub struct EnglishyInput {
    pub bytes: Vec<u8>,
    pub xor_key: Vec<u8>,
}

#[derive(Debug)]
pub struct EnglishyResult {
    pub bytes: Vec<u8>,
    pub xor_key: Vec<u8>,
    pub score: f64,
}

const ASCII_SPACE: u8 = 0x20;
const ASCII_LOWERCASE_A: u8 = 0x61;
const ASCII_LOWERCASE_Z: u8 = 0x7a;
const ASCII_UPPERCASE_A: u8 = 0x41;
const ASCII_UPPERCASE_Z: u8 = 0x5a;

fn score_bytes(bytes: &Vec<u8>) -> f64 {
    let mut score = 0;
    for byte in bytes {
        if *byte == ASCII_SPACE || (
            ASCII_LOWERCASE_A <= *byte && ASCII_LOWERCASE_Z >= *byte
        ) || (
            ASCII_UPPERCASE_A <= *byte && ASCII_UPPERCASE_Z >= *byte
        ) {
            score = score + 1;
        }
    }
    score as f64 / bytes.len() as f64
}

pub fn get_most_englishy(attempts: &Vec<EnglishyInput>) -> EnglishyResult {
    let mut best_score = 0f64;
    let mut best_result = EnglishyResult { bytes: vec![0u8], xor_key: vec![0u8], score: 0f64 };
    for attempt in attempts {
        let score = score_bytes(&attempt.bytes);
        if score > best_score {
            best_score = score;
            best_result = EnglishyResult {
                bytes: attempt.bytes.to_vec(),
                xor_key: attempt.xor_key.to_vec(),
                score,
            };
        }
    }
    best_result
}
