use kolmogorov_smirnov::test;
use ascii::ascii_to_bytes;

const ENGLISH_TRAINING_TEXT: &str = "The Kolmogorov-Smirnov test is a hypothesis test procedure for determining if two samples of data are from the same distribution. The test is non-parametric and entirely agnostic to what this distribution actually is. The fact that we never have to know the distribution the samples come from is incredibly useful, especially in software and operations where the distributions are hard to express and difficult to calculate with. Here is a question? And an exclamation, too!";

#[derive(Debug)]
pub struct EnglishyInput {
    pub bytes: Vec<u8>,
    pub xor_key: Vec<u8>,
}

#[derive(Debug)]
pub struct EnglishyResult {
    pub bytes: Vec<u8>,
    pub xor_key: Vec<u8>,
    pub ks_statistic: f64,
}

pub fn most_englishy(attempts: &Vec<EnglishyInput>) -> EnglishyResult {
    let training_text_parsed = ascii_to_bytes(ENGLISH_TRAINING_TEXT);
    let mut lowest_ks_statistic = 1f64;
    let mut best_result = EnglishyResult { bytes: vec![0u8], xor_key: vec![0u8], ks_statistic: 1f64 };
    for attempt in attempts {
        // confidence value of 0.1 seems to work well, not entirely clear why
        let result = test(&training_text_parsed, &attempt.bytes, 0.1);
        if result.statistic < lowest_ks_statistic {
            lowest_ks_statistic = result.statistic;
            best_result = EnglishyResult {
                bytes: attempt.bytes.to_vec(),
                ks_statistic: result.statistic,
                xor_key: attempt.xor_key.to_vec(),
            };
        }
    }
    best_result
}
