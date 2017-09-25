mod hex;
mod single_byte_xor;
mod ascii;
use single_byte_xor::{guess_single_byte_xor, COMMON_CHARS};

fn main() {
    let input = "1b37373331363f78151b7f2b783431333d78397828372d363c78373e783a393b3736";
    for guess in COMMON_CHARS.iter() {
        let decoded = guess_single_byte_xor(input, *guess);
        println!("guess with {:x}: {}", &guess, decoded);
    }
}
