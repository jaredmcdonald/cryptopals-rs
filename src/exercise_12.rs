use aes_oracles::{detection_oracle, ecb_oracle, find_blocksize, AesEncryptionMode};
use ascii::bytes_to_ascii_string;
use decrypt_ecb::decrypt_ecb;

pub fn run_12() {
    let encrypter = ecb_oracle(false);
    let blocksize = find_blocksize(&encrypter);
    let decrypted = match detection_oracle(&encrypter) {
        AesEncryptionMode::ECB => decrypt_ecb(&encrypter, blocksize),
        _ => panic!("it's not ECB, I don't know how to decrypt this!"),
    };
    println!("decrypted unknown-string:\n{}", bytes_to_ascii_string(&decrypted));
}
