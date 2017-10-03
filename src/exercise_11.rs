use aes_oracles::{encryption_oracle, detection_oracle};

const ITERATIONS: usize = 5;
pub fn run_11() {
    for _ in 0..ITERATIONS {
        let encrypter = encryption_oracle();
        println!("guess: {:?}", detection_oracle(&encrypter));
    }
}
