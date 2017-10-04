use aes_oracles::{ecb_oracle, detection_oracle, find_blocksize, AesEncryptionMode};
use ascii::bytes_to_ascii_string;
use decrypt_ecb::{decrypt_ecb_with_prefix};

pub fn run_14() {
    let encrypter = ecb_oracle(true);
    let blocksize = find_blocksize(&encrypter);
    let decrypted = match detection_oracle(&encrypter) {
        AesEncryptionMode::ECB => decrypt_ecb_with_prefix(&encrypter, blocksize),
        _ => panic!("it's not ECB, I don't know how to decrypt this!"),
    };
    println!("decrypted unknown-string:\n{}", bytes_to_ascii_string(&decrypted));
}
