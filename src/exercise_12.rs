use std::collections::HashMap;
use base64::decode as base64_decode;
use aes::encrypt_aes_ecb;
use aes_oracles::{random_key, detection_oracle, AesEncryptionMode};
use ascii::bytes_to_ascii_string;

fn create_encrypter() -> Box<Fn(&[u8]) -> Vec<u8>> {
    let key = random_key();
    let plaintext_to_append = base64_decode("Um9sbGluJyBpbiBteSA1LjAKV2l0aCBteSByYWctdG9wIGRvd24gc28gbXkgaGFpciBjYW4gYmxvdwpUaGUgZ2lybGllcyBvbiBzdGFuZGJ5IHdhdmluZyBqdXN0IHRvIHNheSBoaQpEaWQgeW91IHN0b3A/IE5vLCBJIGp1c3QgZHJvdmUgYnkK").unwrap();
    Box::new(move |plaintext| {
        let mut extended_plaintext = Vec::new();
        extended_plaintext.extend(plaintext);
        extended_plaintext.extend(&plaintext_to_append);
        encrypt_aes_ecb(&extended_plaintext, &key)
    })
}

fn find_blocksize(encrypter: &Box<Fn(&[u8]) -> Vec<u8>>) -> usize {
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

fn decrypt_byte(
    encrypter: &Box<Fn(&[u8]) -> Vec<u8>>,
    len_with_padding: usize,
    decoded_bytes: &[u8],
    target_index: usize
) -> Option<u8> {
    let mut first_blocks_to_bytes = HashMap::new();
    let short_block = vec![0x0; target_index];
    let mut partially_decoded = short_block.clone();
    partially_decoded.extend(decoded_bytes);
    for byte in 0..0xff {
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

fn decrypt_ecb(encrypter: &Box<Fn(&[u8]) -> Vec<u8>>, blocksize: usize) -> Vec<u8> {
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

pub fn run_12() {
    let encrypter = create_encrypter();
    let blocksize = find_blocksize(&encrypter);
    let decrypted = match detection_oracle(&encrypter) {
        AesEncryptionMode::ECB => decrypt_ecb(&encrypter, blocksize),
        _ => panic!("it's not ECB, I don't know how to decrypt this!"),
    };
    println!("decrypted unknown-string:\n{}", bytes_to_ascii_string(&decrypted));
}
