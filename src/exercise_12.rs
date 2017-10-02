use aes::{random_key, encrypt_aes_ecb};
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

fn find_block_size(encrypter: Box<Fn(&[u8]) -> Vec<u8>>) -> usize {
    let mut block_size = 0;
    let byte = 0x0;
    let smallest_size = encrypter(vec![byte; 0x1].as_slice()).len();
    for n in 0x002..0x100 {
        let result = encrypter(vec![0x0u8; n].as_slice());
        let len = result.len();
        if len != smallest_size {
            block_size = len - smallest_size;
            break;
        }
    }
    block_size
}

pub fn run_12() {
    let encrypter = create_encrypter();
    let block_size = find_block_size(encrypter);
    println!("{}", block_size);
}
