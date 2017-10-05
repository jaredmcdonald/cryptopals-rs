use rand::{Rng, thread_rng};
use base64::decode as base64_decode;
use aes_oracles::random_key;
use pkcs_7::{pad, unpad};
use aes::{encrypt_aes_cbc, decrypt_aes_cbc, BLOCK_SIZE};

fn encrypter(key: &[u8], iv: &[u8]) -> Vec<u8> {
    let b64_strings = vec![
        "MDAwMDAwTm93IHRoYXQgdGhlIHBhcnR5IGlzIGp1bXBpbmc=",
        "MDAwMDAxV2l0aCB0aGUgYmFzcyBraWNrZWQgaW4gYW5kIHRoZSBWZWdhJ3MgYXJlIHB1bXBpbic=",
        "MDAwMDAyUXVpY2sgdG8gdGhlIHBvaW50LCB0byB0aGUgcG9pbnQsIG5vIGZha2luZw==",
        "MDAwMDAzQ29va2luZyBNQydzIGxpa2UgYSBwb3VuZCBvZiBiYWNvbg==",
        "MDAwMDA0QnVybmluZyAnZW0sIGlmIHlvdSBhaW4ndCBxdWljayBhbmQgbmltYmxl",
        "MDAwMDA1SSBnbyBjcmF6eSB3aGVuIEkgaGVhciBhIGN5bWJhbA==",
        "MDAwMDA2QW5kIGEgaGlnaCBoYXQgd2l0aCBhIHNvdXBlZCB1cCB0ZW1wbw==",
        "MDAwMDA3SSdtIG9uIGEgcm9sbCwgaXQncyB0aW1lIHRvIGdvIHNvbG8=",
        "MDAwMDA4b2xsaW4nIGluIG15IGZpdmUgcG9pbnQgb2g=",
        "MDAwMDA5aXRoIG15IHJhZy10b3AgZG93biBzbyBteSBoYWlyIGNhbiBibG93"
    ];
    let plaintext = base64_decode(thread_rng().choose(&b64_strings).unwrap()).unwrap();
    encrypt_aes_cbc(&pad(&plaintext, BLOCK_SIZE), key, iv)
}

fn decrypt_and_check_padding(ciphertext: &[u8], key: &[u8], iv: &[u8]) -> bool {
    let decrypted = decrypt_aes_cbc(ciphertext, key, iv);
    if let Ok(_) = unpad(&decrypted, BLOCK_SIZE) { true } else { false } // 👈 is there a better way to do this?
}

pub fn run_17() {
    let key = random_key();
    let iv = random_key();
    for byte in 0..0x100 {
        let mut ciphertext = encrypter(&key, &iv);
        let len = ciphertext.len();
        ciphertext[len - 1] = byte as u8;
        let result = decrypt_and_check_padding(&ciphertext, &key, &iv);
        // now what??
        println!("{}", result);
    }
}
