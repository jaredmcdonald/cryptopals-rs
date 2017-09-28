use base64::decode;

pub fn flatten_lines(byte_lines: &Vec<Vec<u8>>) -> Vec<u8> {
    let mut bytes = Vec::new();
    for byte in byte_lines {
        bytes.extend(byte);
    }
    bytes
}

pub fn decode_base64_lines(lines: &Vec<String>) -> Vec<Vec<u8>> {
    let mut bytes = Vec::new();
    for line in lines {
        // really need to figure out `map`, `reduce`, etc :/
        bytes.push(decode(line).unwrap());
    }
    bytes
}
