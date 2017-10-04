use aes_oracles::{ecb_or_cbc_oracle, detection_oracle};

const ITERATIONS: usize = 5;
pub fn run_11() {
    for _ in 0..ITERATIONS {
        let encrypter = ecb_or_cbc_oracle();
        println!("guess: {:?}", detection_oracle(&encrypter));
    }
}
