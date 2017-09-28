use openssl::symm::{Crypter, Cipher, Mode};

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
    fn sanity_check() {
        // ... i guess this is working correctly?
        // https://github.com/sfackler/rust-openssl/issues/40#issuecomment-269417798
        let plaintext = "AAAAAAAAAAAAAAAA".as_bytes();
        let key = "BBBBBBBBBBBBBBBB".as_bytes();
        let (ciphertext, cipherlen) = encrypt_aes_ecb(plaintext, key);
        let (decrypted, _) = decrypt_aes_ecb(&ciphertext[..cipherlen], key);
        assert_eq!(decrypted[..cipherlen], *plaintext);
    }
}
