use aes_oracles::{ecb_oracle, detection_oracle, find_blocksize, AesEncryptionMode};
use ascii::bytes_to_ascii_string;
use utils::as_blocks;
use std::collections::HashMap;
use std::collections::hash_map::Entry;

fn find_first_repetitive_block(ciphertext: &[u8], blocksize: usize) -> Option<usize> {
    let blocks = as_blocks(ciphertext, blocksize);
    let mut blocks_to_indices = HashMap::new();
    for (index, block) in blocks.iter().enumerate() {
        match blocks_to_indices.entry(block) {
            Entry::Occupied(entry) => return Some(entry.get() * blocksize),
            Entry::Vacant(entry) => entry.insert(index),
        };
    }
    None
}

fn find_prefix_len(encrypter: &Box<Fn(&[u8]) -> Vec<u8>>, blocksize: usize) -> Option<usize> {
    let repetition_payload = vec![0x0; blocksize * 2];
    for pad_len in 0..blocksize {
        let mut payload = Vec::new();
        payload.extend(vec![0x1; pad_len].as_slice());
        payload.extend(repetition_payload.as_slice());
        let ciphertext = encrypter(payload.as_slice());
        if let Some(repetition_index) = find_first_repetitive_block(&ciphertext, blocksize) {
            return Some(repetition_index - pad_len)
        }
    }
    None
}

fn decrypt_ecb(encrypter: &Box<Fn(&[u8]) -> Vec<u8>>, blocksize: usize) -> Vec<u8> {
    let prefix_len = find_prefix_len(encrypter, blocksize).unwrap();
    println!("prefix_len: {}", prefix_len);
    Vec::new()
}

pub fn run_14() {
    let encrypter = ecb_oracle(true);
    let blocksize = find_blocksize(&encrypter);
    let decrypted = match detection_oracle(&encrypter) {
        AesEncryptionMode::ECB => decrypt_ecb(&encrypter, blocksize),
        _ => panic!("it's not ECB, I don't know how to decrypt this!"),
    };
    println!("decrypted unknown-string:\n{}", bytes_to_ascii_string(&decrypted));
}
