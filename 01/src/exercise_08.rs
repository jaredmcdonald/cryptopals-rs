use read_file::strings_from_filename;
use hex::parse_hex;
use edit_distance::get_edit_distance;
use aes::decrypt_aes_ecb;

fn is_aes_encrypted(bytes: &[u8]) -> bool {
    // ... todo, probably something about edit distances?
    false
}

pub fn run_08() {
    let lines = strings_from_filename("08.txt");
    for line in lines {
        let bytes = parse_hex(&line);
        if is_aes_encrypted(&bytes) {
            println!("this string is AES ECB encrypted:\n{}", line);
        }
    }
}
