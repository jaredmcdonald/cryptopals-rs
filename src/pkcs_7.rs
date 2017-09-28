pub fn pad(bytes: &[u8], block_size: u8) -> Vec<u8> {
    let bytes_len = bytes.len() as u8;
    let padding = block_size - bytes_len % block_size;
    let padding_bytes = if padding != 0 { padding } else { block_size } as usize;
    let mut padded = Vec::new();
    padded.extend(bytes);
    padded.extend(vec![padding; padding_bytes]);
    padded
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn input_less_than_one_block() {
        let input = "YELLOW SUBMARINE".as_bytes();
        let expected = "YELLOW SUBMARINE\x04\x04\x04\x04".as_bytes();
        let output = pad(&input, 20);
        assert_eq!(output.as_slice(), expected);
    }

    #[test]
    fn input_size_of_block() {
        let input = "YELLOW S".as_bytes();
        let expected = "YELLOW S\x08\x08\x08\x08\x08\x08\x08\x08".as_bytes();
        let output = pad(&input, 8);
        assert_eq!(output.as_slice(), expected);
    }

    #[test]
    fn input_multiple_of_block() {
        let input = "YELLOW SUBMARINE".as_bytes();
        let expected = "YELLOW SUBMARINE\x08\x08\x08\x08\x08\x08\x08\x08".as_bytes();
        let output = pad(&input, 8);
        assert_eq!(output.as_slice(), expected);
    }

    #[test]
    fn input_longer_than_block() {
        let input = "YELLOW SUBMARI".as_bytes();
        let expected = "YELLOW SUBMARI\x02\x02".as_bytes();
        let output = pad(&input, 8);
        assert_eq!(output.as_slice(), expected);
    }
}
