use openssl::symm::{Crypter, Cipher, Mode};
use utils::{as_blocks, xor_buffers};

fn aes_ecb(source: &[u8], key: &[u8], mode: Mode) -> (Vec<u8>, usize) {
    let cipher = Cipher::aes_128_ecb();
    let mut crypter = Crypter::new(
        cipher,
        mode,
        &key,
        None
    ).unwrap();

    let mut output = vec![0; source.len() + cipher.block_size()];
    match crypter.update(&source, output.as_mut_slice()) {
        Ok(cipherlen) => (output.to_vec(), cipherlen),
        Err(e) => panic!("aes_ecb error: {:?}", e),
    }
}

pub fn decrypt_aes_cbc(source: &[u8], key: &[u8], iv: &[u8]) -> Vec<Vec<u8>> {
    let blocks = as_blocks(source, 16);
    let mut output: Vec<Vec<u8>> = Vec::new();
    for (index, block) in blocks.iter().enumerate() {
        let (text, _) = decrypt_aes_ecb(&block, key);
        let xor_against = if index == 0 { iv } else { &blocks[index - 1] };
        let output_block = xor_buffers(&text[0..16], &xor_against).to_vec();
        output.push(output_block);
    }
    output
}

pub fn decrypt_aes_ecb(ciphertext: &[u8], key: &[u8]) -> (Vec<u8>, usize) {
    aes_ecb(ciphertext, key, Mode::Decrypt)
}

pub fn encrypt_aes_ecb(text: &[u8], key: &[u8]) -> (Vec<u8>, usize) {
    aes_ecb(text, key, Mode::Encrypt)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ecb_sanity_check() {
        // ... i guess this is working correctly?
        // https://github.com/sfackler/rust-openssl/issues/40#issuecomment-269417798
        let plaintext = "AAAAAAAAAAAAAAAA".as_bytes();
        let key = "BBBBBBBBBBBBBBBB".as_bytes();
        let (ciphertext, cipherlen) = encrypt_aes_ecb(plaintext, key);
        let (decrypted, _) = decrypt_aes_ecb(&ciphertext[..cipherlen], key);
        assert_eq!(decrypted[..cipherlen], *plaintext);
    }
}
