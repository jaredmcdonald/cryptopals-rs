pub fn bytes_to_ascii_string(bytes: &[u8]) -> String {
    bytes.iter().map(|b| *b as char).collect()
}
