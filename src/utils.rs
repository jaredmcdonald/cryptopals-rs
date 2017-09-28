use base64::decode;

pub fn as_blocks(bytes: &[u8], block_size: usize) -> Vec<Vec<u8>> {
    let mut blocks = Vec::new();
    for block_start in 0..bytes.len() / block_size {
        blocks.push(bytes[block_start * block_size..(block_start + 1) * block_size].to_vec())
    }
    blocks
}

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

pub fn xor_buffers(a: &[u8], b: &[u8]) -> Vec<u8> {
    assert!(a.len() == b.len(), "can't xor buffers of unequal length");
    // is there a way to just map this? how to access the byte in b, since map fn doesn't get index?
    let mut xored = Vec::new();
    for i in 0..a.len() {
        xored.push(a[i] ^ b[i]);
    }
    xored
}
