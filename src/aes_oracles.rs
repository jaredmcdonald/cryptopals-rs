use std::collections::HashSet;
use rand::random;
use base64::decode as base64_decode;
use aes::{BLOCK_SIZE, encrypt_aes_cbc, encrypt_aes_ecb_padded};
use utils::{random_bytes, random_bytes_between};

pub type Encrypter<'a> = Box<Fn(&[u8]) -> Vec<u8> + 'a>;

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

pub fn detection_oracle(encrypter: &Encrypter) -> AesEncryptionMode {
    let ciphertext = encrypter(vec![0x0; 0x200].as_slice()); // should be enough to generate repeated blocks
    if is_ecb_encrypted(&ciphertext) {
        AesEncryptionMode::ECB
    } else {
        AesEncryptionMode::CBC
    }
}

fn random_bytes_around(bytes: &[u8]) -> Vec<u8> {
    let mut modified = Vec::new();
    modified.extend(random_bytes_between(5, 11));
    modified.extend(bytes);
    modified.extend(random_bytes_between(5, 11));
    modified
}

pub fn ecb_or_cbc_oracle<'a>() -> Encrypter<'a> {
    Box::new(|plaintext| {
        let modified_plaintext = random_bytes_around(plaintext);
        if random::<bool>() { // CBC
            encrypt_aes_cbc(&modified_plaintext, &random_bytes(BLOCK_SIZE), &random_bytes(BLOCK_SIZE))
        } else { // ECB
            encrypt_aes_ecb_padded(&modified_plaintext, &random_bytes(BLOCK_SIZE))
        }
    })
}

pub fn ecb_oracle<'a>(prepend_random_bytes: bool) -> Encrypter<'a> {
    let key = random_bytes(BLOCK_SIZE);
    let plaintext_to_append = base64_decode("Um9sbGluJyBpbiBteSA1LjAKV2l0aCBteSByYWctdG9wIGRvd24gc28gbXkgaGFpciBjYW4gYmxvdwpUaGUgZ2lybGllcyBvbiBzdGFuZGJ5IHdhdmluZyBqdXN0IHRvIHNheSBoaQpEaWQgeW91IHN0b3A/IE5vLCBJIGp1c3QgZHJvdmUgYnkK").unwrap();
    // 2.14: assuming the prefix is unknown but constant; if it's not constant, this becomes a lot harder
    let garbage_prefix = random_bytes_between(2, 0x100);
    Box::new(move |plaintext| {
        let mut extended_plaintext = Vec::new();
        if prepend_random_bytes {
            extended_plaintext.extend(&garbage_prefix);
        }
        extended_plaintext.extend(plaintext);
        extended_plaintext.extend(&plaintext_to_append);
        encrypt_aes_ecb_padded(&extended_plaintext, &key)
    })
}

pub fn find_blocksize(encrypter: &Encrypter) -> usize {
    let mut blocksize = 0;
    let smallest_size = encrypter(Vec::new().as_slice()).len();
    for n in 0x001..0x100 {
        let result = encrypter(vec![0x0u8; n].as_slice());
        let len = result.len();
        if len != smallest_size {
            blocksize = len - smallest_size;
            break;
        }
    }
    blocksize
}
