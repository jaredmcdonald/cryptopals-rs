use read_file::strings_from_filename;
use utils::{flatten_lines, decode_base64_lines};
use aes::decrypt_aes_cbc;

pub fn run_10() {
    let ciphertext = flatten_lines(&decode_base64_lines(&strings_from_filename("10.txt")));
    let key = "YELLOW SUBMARINE".as_bytes();
    decrypt_aes_cbc(&ciphertext, key, [0x0u8; 16]);
}
