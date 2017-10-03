use openssl::symm::{Crypter, Cipher, Mode};
use utils::{as_blocks, xor_buffers, flatten};

fn aes_ecb(data: &[u8], key: &[u8], mode: Mode) -> Vec<u8> {
    let cipher = Cipher::aes_128_ecb();
    let mut crypter = Crypter::new(
        cipher,
        mode,
        &key,
        None
    ).unwrap();

    // https://github.com/sfackler/rust-openssl/blob/master/openssl/src/symm.rs#L383-L396
    let mut output = vec![0; data.len() + cipher.block_size()];
    let count = crypter.update(data, &mut output).unwrap();
    let rest = crypter.finalize(&mut output[count..]).unwrap();
    output.truncate(count + rest);
    output
}

pub const BLOCK_SIZE: usize = 0x10;

pub fn decrypt_aes_cbc(ciphertext: &[u8], key: &[u8], iv: &[u8]) -> Vec<u8> {
    let blocks = as_blocks(ciphertext, BLOCK_SIZE);
    let mut output: Vec<Vec<u8>> = Vec::new();
    for (index, block) in blocks.iter().enumerate() {
        let text = decrypt_aes_ecb(&block, key);
        let xor_against = if index == 0 { iv } else { &blocks[index - 1] };
        let output_block = xor_buffers(&text[..BLOCK_SIZE], &xor_against).to_vec();
        output.push(output_block);
    }
    flatten(&output)
}

pub fn encrypt_aes_cbc(plaintext: &[u8], key: &[u8], iv: &[u8]) -> Vec<u8> {
    let blocks = as_blocks(plaintext, BLOCK_SIZE);
    let mut output: Vec<Vec<u8>> = Vec::new();
    for (index, block) in blocks.iter().enumerate() {
        let output_copy = output.clone();
        let xor_against = if index == 0 { iv } else { &output_copy[index - 1] };
        let to_encrypt = xor_buffers(block, xor_against);
        let ciphertext = encrypt_aes_ecb(&to_encrypt, key);
        output.push(ciphertext);
    }
    flatten(&output)
}

pub fn decrypt_aes_ecb(ciphertext: &[u8], key: &[u8]) -> Vec<u8> {
    aes_ecb(ciphertext, key, Mode::Decrypt)
}

pub fn encrypt_aes_ecb(text: &[u8], key: &[u8]) -> Vec<u8> {
    aes_ecb(text, key, Mode::Encrypt)
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ecb_sanity_check() {
        // ... i guess this is working correctly?
        // https://github.com/sfackler/rust-openssl/issues/40#issuecomment-269417798
        let plaintext = "AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA".as_bytes();
        let key = "BBBBBBBBBBBBBBBB".as_bytes();
        let ciphertext = encrypt_aes_ecb(plaintext, key);
        let decrypted = decrypt_aes_ecb(&ciphertext, key);
        assert_eq!(decrypted[..plaintext.len()], *plaintext); // eh... unpad?
    }

    #[test]
    fn cbc_sanity_check() {
        let plaintext = "ABABABABABAXAXAAXAADAADAADAACAACACACACACACACACAC".as_bytes();
        let key = "BBBBBBBBBBBBBBBB".as_bytes();
        let iv = "CCCCCCCCCCCCCCCC".as_bytes();
        let ciphertext = encrypt_aes_cbc(plaintext, key, &iv);
        let decrypted = decrypt_aes_cbc(&ciphertext, key, &iv);
        assert_eq!(decrypted, plaintext.to_vec());
    }
}
