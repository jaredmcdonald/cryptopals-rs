use read_file::strings_from_filename;
use hex::parse_hex;
use aes::is_ecb_encrypted;

pub fn run_08() {
    let lines = strings_from_filename("08.txt");
    for line in lines {
        let bytes = parse_hex(&line);
        if is_ecb_encrypted(&bytes) {
            println!("this string is AES ECB encrypted:\n{}", line);
        }
    }
}
