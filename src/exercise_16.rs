use aes::{encrypt_aes_cbc, decrypt_aes_cbc, BLOCK_SIZE};
use aes_oracles::random_bytes;
use utils::xor_buffers;
use pkcs_7::pad;
use ascii::bytes_to_ascii_string;

fn generate_profile(content: &[u8]) -> Vec<u8> {
    format!("comment1=cooking%20MCs;userdata={};comment2=%20like%20a%20pound%20of%20bacon",
        bytes_to_ascii_string(content).replace(";", "%3B").replace("=", "%3D").replace(" ", "%20")
    ).as_bytes().to_vec()
}

fn generate_encrypted_profile(content: &[u8], key: &[u8], iv: &[u8]) -> Vec<u8> {
    encrypt_aes_cbc(&pad(&generate_profile(content), BLOCK_SIZE), key, iv)
}

pub fn run_16() {
    let key = random_bytes(BLOCK_SIZE);
    let iv = random_bytes(BLOCK_SIZE);

    let target = "ha;admin=true;ha".as_bytes();
    let input = "0123456789abcdef".as_bytes();

    let ciphertext = generate_encrypted_profile(input, &key, &iv);
    let real_decrypted = decrypt_aes_cbc(&ciphertext, &key, &iv);

    let attack_payload = xor_buffers(target, &real_decrypted[BLOCK_SIZE..BLOCK_SIZE * 2]);
    let mut manipulated_ciphertext = xor_buffers(&ciphertext[..BLOCK_SIZE], &attack_payload);
    manipulated_ciphertext.extend(&ciphertext[BLOCK_SIZE..]);

    let manipulated_decrypted = decrypt_aes_cbc(&manipulated_ciphertext, &key, &iv);
    let as_string = bytes_to_ascii_string(&manipulated_decrypted);
    println!("result:\n{}", as_string);
    println!("success? {}", as_string.contains(";admin=true;"));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn interpolates_in_content() {
        assert_eq!(generate_profile(b"hello world"),
            b"comment1=cooking%20MCs;userdata=hello%20world;comment2=%20like%20a%20pound%20of%20bacon".to_vec());
    }

    #[test]
    fn quotes_metacharacters() {
        assert_eq!(generate_profile(b"hello world;role=admin"),
            b"comment1=cooking%20MCs;userdata=hello%20world%3Brole%3Dadmin;comment2=%20like%20a%20pound%20of%20bacon".to_vec());
    }
}
