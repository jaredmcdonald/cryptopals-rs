use std::collections::HashMap;

fn decrypt_byte(
    encrypter: &Box<Fn(&[u8]) -> Vec<u8>>,
    len_with_padding: usize,
    decoded_bytes: &[u8],
    target_index: usize
) -> Option<u8> {
    let mut first_blocks_to_bytes = HashMap::new();
    let short_block = vec![0x0; target_index];
    let mut partially_decoded = short_block.clone();
    partially_decoded.extend(decoded_bytes);
    for byte in 0..0xff {
        let mut input = partially_decoded.clone();
        input.push(byte);
        let first_output_block = encrypter(input.as_slice())[..len_with_padding].to_vec();
        first_blocks_to_bytes.insert(first_output_block, byte);
    }
    let actual_first_byte = match encrypter(&short_block).get(..len_with_padding) {
        Some(byte) => byte.to_vec(),
        None => return None,
    };
    match first_blocks_to_bytes.get(&actual_first_byte) {
        Some(byte) => Some(*byte),
        None => None,
    }
}

pub fn decrypt_ecb(encrypter: &Box<Fn(&[u8]) -> Vec<u8>>, blocksize: usize) -> Vec<u8> {
    let unknown_str_len = encrypter(vec![].as_slice()).len() + blocksize; // add blocksize for padding
    let mut decrypted_bytes = Vec::new();
    for byte_n in 0..unknown_str_len {
        let num_unknowns = unknown_str_len - (byte_n + 1);
        let partially_decoded = decrypted_bytes.clone();
        match decrypt_byte(&encrypter, unknown_str_len, partially_decoded.as_slice(), num_unknowns) {
            Some(byte) => decrypted_bytes.push(byte),
            None => break,
        }
    }
    decrypted_bytes
}
