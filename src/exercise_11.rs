use rand::{random, thread_rng, Rng};
use aes::{BLOCK_SIZE, encrypt_aes_cbc, encrypt_aes_ecb, is_ecb_encrypted};

fn random_key() -> [u8; BLOCK_SIZE] {
    random::<[u8; BLOCK_SIZE]>()
}

fn random_bytes() -> Vec<u8> {
    let num_extra_bytes: usize = thread_rng().gen_range(5, 11);
    let mut output = Vec::new();
    for _ in 0..num_extra_bytes {
        output.push(random::<u8>());
    }
    output
}

fn random_bytes_around(bytes: &[u8]) -> Vec<u8> {
    let mut modified = Vec::new();
    modified.extend(random_bytes());
    modified.extend(bytes);
    modified.extend(random_bytes());
    modified
}

fn encryption_oracle(plaintext: &[u8]) -> Vec<u8> {
    let modified_plaintext = random_bytes_around(plaintext);

    if random::<bool>() { // CBC
        println!("CBC");
        encrypt_aes_cbc(&modified_plaintext, &random_key(), &random_key())
    } else { // ECB
        println!("ECB");
        encrypt_aes_ecb(&modified_plaintext, &random_key())
    }
}

#[derive(Debug)]
enum AesEncryptionMode {
    CBC,
    ECB,
}

fn detection_oracle(ciphertext: &[u8]) -> AesEncryptionMode {
    if is_ecb_encrypted(ciphertext) {
        AesEncryptionMode::ECB
    } else {
        AesEncryptionMode::CBC
    }
}

const ITERATIONS: usize = 5;
pub fn run_11() {
    for _ in 0..ITERATIONS {
        // am i supposed to be able to manipulate what this thing recieves, or what?
        let enc = encryption_oracle("ABC123ABC123ABC123ABC123ABC123ABC123ABC123ABC123ABC123ABC123ABC123ABC123ABC123".as_bytes());
        println!("guess: {:?}", detection_oracle(&enc));
    }
}
