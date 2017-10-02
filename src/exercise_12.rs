use std::collections::HashMap;
use aes::encrypt_aes_ecb;
use aes_oracles::{random_key, detection_oracle};
use base64::decode as base64_decode;

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
    let byte = 0x0;
    let smallest_size = encrypter(vec![byte; 0x1].as_slice()).len();
    for n in 0x002..0x100 {
        let result = encrypter(vec![0x0u8; n].as_slice());
        let len = result.len();
        if len != smallest_size {
            blocksize = len - smallest_size;
            break;
        }
    }
    blocksize
}

fn find_first_byte(encrypter: &Box<Fn(&[u8]) -> Vec<u8>>, blocksize: usize) -> u8 {
    let mut first_blocks_to_bytes = HashMap::new();
    let short_block = vec![0x0; blocksize - 1];
    for byte in 0..0xff {
        let mut input = short_block.clone();
        input.push(byte);
        let first_output_block = encrypter(input.as_slice())[..blocksize].to_vec();
        first_blocks_to_bytes.insert(first_output_block, byte);
    }
    let actual_first_byte = encrypter(short_block.as_slice())[..blocksize].to_vec();
    *first_blocks_to_bytes.get(&actual_first_byte).unwrap()
}

pub fn run_12() {
    let encrypter = create_encrypter();
    let blocksize = find_blocksize(&encrypter);
    let mode = detection_oracle(&encrypter);
    let first_byte = find_first_byte(&encrypter, blocksize);
    println!("{}, {:?}", first_byte, mode);
}
