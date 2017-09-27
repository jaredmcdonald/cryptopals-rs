use openssl::symm::{Crypter, Cipher, Mode};

pub fn decrypt_aes_ecb(ciphertext: &[u8], key: &[u8]) -> Vec<u8> { // how to return a slice?
    let cipher = Cipher::aes_128_ecb();                            // punt and just use Vec for now
    let mut decrypter = Crypter::new(
        cipher,
        Mode::Decrypt,
        &key,
        None // not really sure what `iv` is but seems ECB mode doesn't need it :/
    ).unwrap();

    let mut decrypted = vec![0; ciphertext.len() + cipher.block_size()];
    if let Err(e) = decrypter.update(&ciphertext, &mut decrypted) {
        println!("error! {:?}", e);
    }
    decrypted.to_vec()
}
