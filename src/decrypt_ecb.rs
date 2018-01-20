use std::collections::HashMap;
use std::collections::hash_map::Entry;
use utils::as_blocks;
use aes_oracles::Encrypter;

fn decrypt_byte(
    encrypter: &Encrypter,
    len_with_padding: usize,
    decoded_bytes: &[u8],
    target_index: usize
) -> Option<u8> {
    let mut first_blocks_to_bytes = HashMap::new();
    let short_block = vec![0x0; target_index];
    let mut partially_decoded = short_block.clone();
    partially_decoded.extend(decoded_bytes);
    for byte in 0..=<u8>::max_value() {
        let mut input = partially_decoded.clone();
        input.push(byte);
        let first_output_block = encrypter(input.as_slice())[..len_with_padding].to_vec();
        first_blocks_to_bytes.insert(first_output_block, byte);
    }
    let actual_first_byte = match encrypter(&short_block).get(..len_with_padding) {
        Some(byte) => byte.to_vec(),
        None => return None,
    };
    match first_blocks_to_bytes.get(&actual_first_byte) {
        Some(byte) => Some(*byte),
        None => None,
    }
}

pub fn decrypt_ecb(encrypter: &Encrypter, blocksize: usize) -> Vec<u8> {
    let unknown_str_len = encrypter(vec![].as_slice()).len() + blocksize; // add blocksize for padding
    let mut decrypted_bytes = Vec::new();
    for byte_n in 0..unknown_str_len {
        let num_unknowns = unknown_str_len - (byte_n + 1);
        let partially_decoded = decrypted_bytes.clone();
        match decrypt_byte(&encrypter, unknown_str_len, partially_decoded.as_slice(), num_unknowns) {
            Some(byte) => decrypted_bytes.push(byte),
            None => break,
        }
    }
    decrypted_bytes
}

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

fn find_prefix_len(encrypter: &Encrypter, blocksize: usize) -> Option<usize> {
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

fn encrypter_factory<'a>(
    old_encrypter: &'a Encrypter,
    prefix_len: usize,
    padding_len: usize
) -> Encrypter<'a> {
    // the number of bytes that are just prefix + prefix padding and should be lopped off each time
    let bytes_to_remove = prefix_len + padding_len;
    Box::new(move |plaintext| {
        let mut modified_plaintext = vec![0x0; padding_len];
        modified_plaintext.extend(plaintext);
        old_encrypter(modified_plaintext.as_slice())[bytes_to_remove..].to_vec()
    })
}

pub fn decrypt_ecb_with_prefix(encrypter: &Encrypter, blocksize: usize) -> Vec<u8> {
    let prefix_len = find_prefix_len(encrypter, blocksize).unwrap();
    // the number of bytes we need to get the prefix to occupy an even number of blocks
    let padding_len = blocksize - (prefix_len % blocksize);
    let new_encrypter = encrypter_factory(encrypter, prefix_len, padding_len);
    decrypt_ecb(&new_encrypter, blocksize)
}
