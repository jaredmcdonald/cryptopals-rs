use read_file::strings_from_filename;
use hex::parse_hex;
use std::collections::HashSet;

const BLOCK_SIZE: usize = 16;

fn is_aes_encrypted(bytes: &[u8]) -> bool {
    let mut blocks = Vec::new();
    for block_start in 0..bytes.len() / BLOCK_SIZE {
        // break into blocks
        blocks.push(bytes[block_start * BLOCK_SIZE..(block_start + 1) * BLOCK_SIZE].to_vec())
    }
    let unique: HashSet<_> = blocks.iter().cloned().collect();
    unique.len() != blocks.len()
}

pub fn run_08() {
    let lines = strings_from_filename("08.txt");
    for line in lines {
        let bytes = parse_hex(&line);
        if is_aes_encrypted(&bytes) {
            println!("this string is AES ECB encrypted:\n{}", line);
        }
    }
}
