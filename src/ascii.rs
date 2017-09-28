pub fn bytes_to_ascii_string(bytes: &[u8]) -> String {
    bytes.iter().map(|b| *b as char).collect()
}

// eventually remove this, easier to just call `as_bytes`
pub fn ascii_to_bytes(ascii: &str) -> Vec<u8> {
    ascii.as_bytes().to_vec()
}
