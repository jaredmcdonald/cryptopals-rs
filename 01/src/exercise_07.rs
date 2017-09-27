use openssl::symm::{Crypter, Cipher, Mode};
use ascii::{ascii_to_bytes, bytes_to_ascii_string};
use read_file::strings_from_filename;
use utils::{flatten_lines, decode_base64_lines};

pub fn run_07() {
    let key = ascii_to_bytes("YELLOW SUBMARINE");
    let ciphertext = flatten_lines(&decode_base64_lines(&strings_from_filename("07.txt")));
    let cipher = Cipher::aes_128_ecb();
    let mut decrypter = Crypter::new(
        cipher,
        Mode::Decrypt,
        &key,
        None // not really sure what `iv` is
    ).unwrap();

    let mut decrypted = vec![0; ciphertext.len() + cipher.block_size()];
    match decrypter.update(&ciphertext, &mut decrypted) {
        _ => println!("{}", bytes_to_ascii_string(&decrypted)),
    }
}
