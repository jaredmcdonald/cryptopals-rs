use kolmogorov_smirnov::test;
use ascii::ascii_to_bytes;

const ENGLISH_TRAINING_TEXT: &str = "The Kolmogorov-Smirnov test is a hypothesis test procedure for determining if two samples of data are from the same distribution. The test is non-parametric and entirely agnostic to what this distribution actually is. The fact that we never have to know the distribution the samples come from is incredibly useful, especially in software and operations where the distributions are hard to express and difficult to calculate with.";

pub struct EnglishyInput {
    pub bytes: Vec<u8>,
    pub xor_key: u8,
}

pub struct EnglishyResult {
    pub bytes: Vec<u8>,
    pub xor_key: u8,
    pub test_result: test::TestResult,
}

pub fn most_englishy(attempts: &Vec<EnglishyInput>) -> Vec<EnglishyResult> {
    let training_text_parsed = ascii_to_bytes(ENGLISH_TRAINING_TEXT);
    let mut passing = Vec::new();
    for attempt in attempts {
        // confidence value of 0.1 seems to work well, not entirely clear why
        let result = test(&training_text_parsed, &attempt.bytes, 0.1);
        if !result.is_rejected {
            passing.push(EnglishyResult {
                bytes: attempt.bytes.to_vec(),
                test_result: result,
                xor_key: attempt.xor_key,
            });
        }
    }
    passing
}

pub fn single_byte_xor(bytes: &Vec<u8>, xor_byte: u8) -> Vec<u8> {
    bytes.iter().map(|byte| byte ^ xor_byte).collect()
}
