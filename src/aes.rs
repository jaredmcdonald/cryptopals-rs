use openssl::symm::{Crypter, Cipher, Mode};
use utils::{as_blocks, xor_buffers, flatten};
use pkcs_7::{pad, unpad};
use byteorder::{LittleEndian, WriteBytesExt};

fn aes_ecb(data: &[u8], key: &[u8], mode: Mode) -> Vec<u8> {
    let cipher = Cipher::aes_128_ecb();
    let mut crypter = Crypter::new(
        cipher,
        mode,
        &key,
        None
    ).unwrap();

    crypter.pad(false);

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
        let output_block = xor_buffers(&text, &xor_against).to_vec();
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

pub fn aes_ctr(text: &[u8], key: &[u8], nonce: u64) -> Vec<u8> {
    let mut output = vec![];
    let mut text_iter = text.iter();
    let mut counter = 0u64;

    let mut nonce_bytes = vec![];
    nonce_bytes.write_u64::<LittleEndian>(nonce).unwrap();

    loop {
        let block_ish: Vec<u8> = text_iter.by_ref().take(BLOCK_SIZE).map(|b| *b).collect();
        let last_iter = block_ish.len() < BLOCK_SIZE;

        let mut keystream = nonce_bytes.clone();
        let mut counter_bytes = vec![];
        counter_bytes.write_u64::<LittleEndian>(counter).unwrap();
        keystream.extend(&counter_bytes);

        let keystream_enc = encrypt_aes_ecb(&keystream, key);
        output.extend(xor_buffers(&block_ish, &keystream_enc));

        if last_iter {
            break;
        }

        counter += 1;
    }
    output
}

pub fn decrypt_aes_ecb(ciphertext: &[u8], key: &[u8]) -> Vec<u8> {
    aes_ecb(ciphertext, key, Mode::Decrypt)
}

pub fn decrypt_aes_ecb_padded(ciphertext: &[u8], key: &[u8]) -> Vec<u8> {
    unpad(&decrypt_aes_ecb(ciphertext, key), BLOCK_SIZE).unwrap()
}

pub fn encrypt_aes_ecb(text: &[u8], key: &[u8]) -> Vec<u8> {
    aes_ecb(text, key, Mode::Encrypt)
}

pub fn encrypt_aes_ecb_padded(text: &[u8], key: &[u8]) -> Vec<u8> {
    encrypt_aes_ecb(&pad(text, BLOCK_SIZE), key)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ecb_sanity_check() {
        // ... i guess this is working correctly?
        // https://github.com/sfackler/rust-openssl/issues/40#issuecomment-269417798
        let plaintext = b"AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA";
        let key = b"BBBBBBBBBBBBBBBB";
        let ciphertext = encrypt_aes_ecb(plaintext, key);
        let decrypted = decrypt_aes_ecb(&ciphertext, key);
        assert_eq!(decrypted[..plaintext.len()], *plaintext); // eh... unpad?
    }

    #[test]
    fn cbc_sanity_check() {
        let plaintext = b"ABABABABABAXAXAAXAADAADAADAACAACACACACACACACACAC";
        let key = b"BBBBBBBBBBBBBBBB";
        let iv = b"CCCCCCCCCCCCCCCC";
        let ciphertext = encrypt_aes_cbc(plaintext, key, iv);
        let decrypted = decrypt_aes_cbc(&ciphertext, key, iv);
        assert_eq!(decrypted, plaintext.to_vec());
    }

    #[test]
    fn ctr_sanity_check() {
        let plaintext = b"abc123 not an even multiple of sixteen bytes";
        let key = b"YELLOW SUBMARINE";
        let nonce = 12345;
        let ciphertext = aes_ctr(plaintext, key, nonce);
        let decrypted = aes_ctr(&ciphertext, key, nonce);
        assert_eq!(decrypted, plaintext.to_vec());
    }
}
