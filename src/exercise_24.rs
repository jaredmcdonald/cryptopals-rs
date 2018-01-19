use rand::random;
use utils::{random_bytes_between, now};
use mersenne_twister::{mt19337_stream_cipher, MersenneTwister};

fn find_seed_brute_force(ciphertext: &[u8], known_plaintext: &[u8]) -> Option<u16> {
    let known_plaintext_start_index = ciphertext.len() - known_plaintext.len();
    for potential_seed in 0..<u16>::max_value() {
        let output = mt19337_stream_cipher(potential_seed, ciphertext);
        if &output[known_plaintext_start_index..] == known_plaintext {
            return Some(potential_seed);
        }
    }
    None
}

// check a token to see if it's from a prng seeded w/ some given timestamp
fn check_token(timestamp: u32, token: u32) -> bool {
    let mut prng = MersenneTwister::new();
    prng.seed(timestamp);
    for _ in 0..10000 { // how can we cap this? 10000 is low, and we can't do the whole prng period
        if prng.rand() == token {
            return true;
        }
    }
    false
}

pub fn run_24() {
    // pt 1
    let known_plaintext = b"AAAAAAAAAAAAAA";
    let mut plaintext = random_bytes_between(5, 20);
    plaintext.extend(known_plaintext);

    let seed = random::<u16>();
    let ciphertext = mt19337_stream_cipher(seed, &plaintext);

    let found_seed = find_seed_brute_force(&ciphertext, known_plaintext).unwrap();
    println!("found seed {}", found_seed);
    println!("correct? {}", found_seed == seed);

    // pt 2, kinda underspecified, is the function supposed to know what the timestamp is?
    let unix_timestamp = now();
    let mut prng = MersenneTwister::new();
    prng.seed(unix_timestamp);

    for _ in 0..1000 {
        let token = prng.rand();
        assert!(check_token(unix_timestamp, token));
    }
    println!("pt 2: ok");
}
