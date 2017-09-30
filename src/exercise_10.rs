use read_file::strings_from_filename;
use utils::{flatten, decode_base64_lines};
use aes::decrypt_aes_cbc;
use ascii::bytes_to_ascii_string;

pub fn run_10() {
    let ciphertext = flatten(&decode_base64_lines(&strings_from_filename("10.txt")));
    let key = "YELLOW SUBMARINE".as_bytes();
    println!("{:?}",
        bytes_to_ascii_string(&flatten(&decrypt_aes_cbc(&ciphertext, key, vec![0x0u8; 16].as_slice()))));
}
