use ascii::bytes_to_ascii_string;
use read_file::strings_from_filename;
use utils::{flatten_lines, decode_base64_lines};
use aes::decrypt_aes_ecb;

pub fn run_07() {
    let key = "YELLOW SUBMARINE".as_bytes();
    let ciphertext = flatten_lines(&decode_base64_lines(&strings_from_filename("07.txt")));
    let (decrypted, _) = decrypt_aes_ecb(&ciphertext, key);
    println!("{}", bytes_to_ascii_string(&decrypted));
}
