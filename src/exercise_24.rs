use rand::random;
use utils::random_bytes_between;
use mersenne_twister::mt19337_stream_cipher;

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

pub fn run_24() {
    let known_plaintext = b"AAAAAAAAAAAAAA";
    let mut plaintext = random_bytes_between(5, 20);
    plaintext.extend(known_plaintext);

    let seed = random::<u16>();
    let ciphertext = mt19337_stream_cipher(seed, &plaintext);

    let found_seed = find_seed_brute_force(&ciphertext, known_plaintext).unwrap();
    println!("found seed {}", found_seed);
    println!("correct? {}", found_seed == seed);
}
