use aes::{encrypt_aes_cbc, decrypt_aes_cbc, BLOCK_SIZE};
use utils::{random_bytes, xor_buffers};

fn encrypt_cbc_iv_key(key: &[u8], plaintext: &[u8]) -> Vec<u8> {
    encrypt_aes_cbc(plaintext, key, key)
}

fn decrypt_cbc_iv_key(key: &[u8], ciphertext: &[u8]) -> Result<Vec<u8>, Vec<u8>> {
    let plaintext = decrypt_aes_cbc(ciphertext, key, key);
    if check_plaintext(&plaintext) {
        Ok(plaintext)
    } else {
        Err(plaintext)
    }
}

fn check_plaintext(plaintext: &[u8]) -> bool {
    plaintext.iter().all(|b| *b < 128)
}

pub fn run_27() {
    let pt = b"AAAAAAAAAAAAAAAABBBBBBBBBBBBBBBBCCCCCCCCCCCCCCCC";
    let key = random_bytes(BLOCK_SIZE);

    let ct = encrypt_cbc_iv_key(&key, pt);

    let mut tampered_ct = ct.clone();
    for i in BLOCK_SIZE..BLOCK_SIZE * 2 {
        // set the second block to be all zeros
        tampered_ct[i] = 0;
    }
    for i in BLOCK_SIZE * 2..BLOCK_SIZE * 3 {
        // set the third block to equal the first block
        tampered_ct[i] = tampered_ct[i - BLOCK_SIZE * 2];
    }
    if let Err(decrypted_tampered_pt) = decrypt_cbc_iv_key(&key, &tampered_ct) {
        let recovered_key = xor_buffers(
            &decrypted_tampered_pt[..BLOCK_SIZE],
            &decrypted_tampered_pt[BLOCK_SIZE * 2..]
        );
        println!("recovered key {:?}", key);
        println!("correct? {}", recovered_key == key);
    } else {
        eprintln!("no error, wat");
    }
}
