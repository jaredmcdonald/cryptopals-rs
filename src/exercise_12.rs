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

pub fn run_12() {
    let encrypter = create_encrypter();
    println!("{:?}", encrypter("AAAAAAAAAAAAAAAA".as_bytes()));
}
