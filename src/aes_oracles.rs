use std::collections::HashSet;
use rand::{random, thread_rng, Rng};
use aes::{BLOCK_SIZE, encrypt_aes_cbc, encrypt_aes_ecb_padded};

pub fn random_key() -> [u8; BLOCK_SIZE] {
    random::<[u8; BLOCK_SIZE]>()
}

pub fn is_ecb_encrypted(bytes: &[u8]) -> bool {
    let mut blocks = Vec::new();
    for block_start in 0..bytes.len() / BLOCK_SIZE {
        // break into blocks
        blocks.push(bytes[block_start * BLOCK_SIZE..(block_start + 1) * BLOCK_SIZE].to_vec())
    }
    let unique: HashSet<_> = blocks.iter().cloned().collect();
    unique.len() != blocks.len()
}

#[derive(Debug)]
pub enum AesEncryptionMode {
    CBC,
    ECB,
}

pub fn detection_oracle(encrypter: &Box<Fn(&[u8]) -> Vec<u8>>) -> AesEncryptionMode {
    let ciphertext = encrypter(vec![0x0; 0x200].as_slice()); // should be enough to generate repeated blocks
    if is_ecb_encrypted(&ciphertext) {
        AesEncryptionMode::ECB
    } else {
        AesEncryptionMode::CBC
    }
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

pub fn encryption_oracle() -> Box<Fn(&[u8]) -> Vec<u8>> {
    Box::new(|plaintext| {
        let modified_plaintext = random_bytes_around(plaintext);
        if random::<bool>() { // CBC
            encrypt_aes_cbc(&modified_plaintext, &random_key(), &random_key())
        } else { // ECB
            encrypt_aes_ecb_padded(&modified_plaintext, &random_key())
        }
    })
}
