mod ascii;
mod hex;
use ascii::ascii_to_bytes;
use hex::as_hex;

fn repeating_key_xor(to_encrypt: &Vec<u8>, key: &Vec<u8>) -> Vec<u8> {
    let mut encrypted = Vec::new();
    let key_len = key.len();
    for i in 0..to_encrypt.len() {
        encrypted.push(to_encrypt[i] ^ key[i % key_len]);
    }
    encrypted
}

fn main() {
    let input = ascii_to_bytes("Burning 'em, if you ain't quick and nimble\nI go crazy when I hear a cymbal");
    let key = ascii_to_bytes("ICE");
    println!("{}", as_hex(&repeating_key_xor(&input, &key)));
}
