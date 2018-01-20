use rand::{Rng, thread_rng};
use base64::decode as base64_decode;
use pkcs_7::{pad, unpad};
use aes::{encrypt_aes_cbc, decrypt_aes_cbc, BLOCK_SIZE};
use utils::{as_blocks, xor_buffers, random_bytes};

fn encrypter(key: &[u8], iv: &[u8]) -> Vec<u8> {
    let b64_strings = [
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
    println!("expected: {}", plaintext.iter().map(|b| *b as char).collect::<String>());
    encrypt_aes_cbc(&pad(&plaintext, BLOCK_SIZE), key, iv)
}

fn decrypt_block(
    block: &[u8],
    previous_block: &[u8],
    padding_oracle: &Fn(&[u8], &[u8]) -> bool
) -> [u8; 16] {
    let mut decoded = [0u8; BLOCK_SIZE];
    let already_has_valid_padding = padding_oracle(previous_block, block);

    for padding_byte in 1..BLOCK_SIZE + 1 { // 1..16 inclusive (TODO)
        let target_byte_index = BLOCK_SIZE - padding_byte; // 15..0 inclusive

        let mut padding_mask = vec![0u8; target_byte_index];
        padding_mask.extend(vec![padding_byte as u8; padding_byte]);
        let base_iv = xor_buffers(&xor_buffers(previous_block, &padding_mask), &decoded);

        for byte in 0x0..=<u8>::max_value() {
            let mut manipulated_iv = base_iv.clone();
            manipulated_iv[target_byte_index] ^= byte;

            if padding_oracle(&manipulated_iv, block) {
                decoded[target_byte_index] = byte;
                if !already_has_valid_padding {
                    break;
                }
            }
        }
    }
    decoded
}

pub fn run_17() {
    let key = random_bytes(BLOCK_SIZE);
    let real_iv = random_bytes(BLOCK_SIZE);
    let ciphertext = encrypter(&key, &real_iv);

    let padding_oracle = |iv: &[u8], ciphertext: &[u8]| -> bool {
        let decrypted = decrypt_aes_cbc(ciphertext, &key, &iv);
        unpad(&decrypted, BLOCK_SIZE).is_ok()
    };

    let mut ciphertext_blocks = as_blocks(&ciphertext, BLOCK_SIZE);
    ciphertext_blocks.reverse();
    ciphertext_blocks.push(real_iv.clone());

    let mut reversed_blocks_peekable = ciphertext_blocks.iter().peekable();

    let mut decrypted = vec![];
    while let Some(ciphertext_block) = reversed_blocks_peekable.next() {
        if let Some(previous_block) = reversed_blocks_peekable.peek() {
            decrypted.push(decrypt_block(
                &ciphertext_block,
                &previous_block,
                &padding_oracle
            ))
        }
    }
    decrypted.reverse();
    println!("decrypted: {:?}", decrypted.iter().flat_map(|f| f).map(|b| *b as char).collect::<String>());
}
