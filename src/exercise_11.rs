use aes_oracles::{encryption_oracle, detection_oracle};

const ITERATIONS: usize = 5;
pub fn run_11() {
    for _ in 0..ITERATIONS {
        // am i supposed to be able to manipulate what this thing recieves, or what?
        let encrypter = encryption_oracle();
        println!("guess: {:?}", detection_oracle(&encrypter));
    }
}
