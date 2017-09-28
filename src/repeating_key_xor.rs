pub fn repeating_key_xor(bytes: &[u8], key: &[u8]) -> Vec<u8> {
    let mut xored = Vec::new();
    let key_len = key.len();
    for i in 0..bytes.len() {
        xored.push(bytes[i] ^ key[i % key_len]);
    }
    xored
}
