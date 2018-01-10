use rand::{Rng, thread_rng};
use base64::decode as base64_decode;
use aes_oracles::random_bytes;
use pkcs_7::{pad, unpad};
use aes::{encrypt_aes_cbc, decrypt_aes_cbc, BLOCK_SIZE};
use utils::as_blocks;

fn encrypter(key: &[u8], iv: &[u8]) -> Vec<u8> {
    let b64_strings = [
        // "MDAwMDAwTm93IHRoYXQgdGhlIHBhcnR5IGlzIGp1bXBpbmc=",
        "MDAwMDAxV2l0aCB0aGUgYmFzcyBraWNrZWQgaW4gYW5kIHRoZSBWZWdhJ3MgYXJlIHB1bXBpbic=",
        // "MDAwMDAyUXVpY2sgdG8gdGhlIHBvaW50LCB0byB0aGUgcG9pbnQsIG5vIGZha2luZw==",
        // "MDAwMDAzQ29va2luZyBNQydzIGxpa2UgYSBwb3VuZCBvZiBiYWNvbg==",
        // "MDAwMDA0QnVybmluZyAnZW0sIGlmIHlvdSBhaW4ndCBxdWljayBhbmQgbmltYmxl",
        // "MDAwMDA1SSBnbyBjcmF6eSB3aGVuIEkgaGVhciBhIGN5bWJhbA==",
        // "MDAwMDA2QW5kIGEgaGlnaCBoYXQgd2l0aCBhIHNvdXBlZCB1cCB0ZW1wbw==",
        // "MDAwMDA3SSdtIG9uIGEgcm9sbCwgaXQncyB0aW1lIHRvIGdvIHNvbG8=",
        // "MDAwMDA4b2xsaW4nIGluIG15IGZpdmUgcG9pbnQgb2g=",
        // "MDAwMDA5aXRoIG15IHJhZy10b3AgZG93biBzbyBteSBoYWlyIGNhbiBibG93"
    ];
    let plaintext = base64_decode(thread_rng().choose(&b64_strings).unwrap()).unwrap();
    encrypt_aes_cbc(&pad(&plaintext, BLOCK_SIZE), key, iv)
}

fn decrypt_block(
    block: &[u8],
    previous_block: &[u8],
    padding_oracle: &Fn(&[u8], &[u8]) -> bool
) -> [u8; BLOCK_SIZE] {
    let faux_block = random_bytes(BLOCK_SIZE);
    let mut found = [0u8; BLOCK_SIZE];
    for byte in 0x1..0xff {
        let mut faux_iv = faux_block.clone();
        faux_iv[BLOCK_SIZE - 1] = byte;

        if padding_oracle(&faux_iv, block) {
            // ? todo
            let intermediate_byte = byte ^ 1;
            found[BLOCK_SIZE - 1] = previous_block[BLOCK_SIZE - 1] ^ intermediate_byte;
        }
    }
    found
}

pub fn run_17() {
    let key = random_bytes(BLOCK_SIZE);
    let real_iv = random_bytes(BLOCK_SIZE);
    let ciphertext = encrypter(&key, &real_iv);
    let len = ciphertext.len();

    let padding_oracle = |iv: &[u8], ciphertext: &[u8]| -> bool {
        let decrypted = decrypt_aes_cbc(ciphertext, &key, &iv);
        unpad(&decrypted, BLOCK_SIZE).is_ok()
    };

    println!("{:?}", decrypt_block(
        &ciphertext[len - BLOCK_SIZE..],
        &ciphertext[len - BLOCK_SIZE * 2..len - BLOCK_SIZE],
        &padding_oracle
    ));
}
