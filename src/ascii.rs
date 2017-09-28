pub fn bytes_to_ascii_string(bytes: &Vec<u8>) -> String {
    bytes.iter().map(|b| *b as char).collect()
}

pub fn ascii_to_bytes(ascii: &str) -> Vec<u8> {
    // is there a way to do this in one line?
    let mut bytes = Vec::new();
    bytes.extend(ascii.chars().map(|c| c as u8));
    bytes
}
