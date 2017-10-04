pub fn pad(bytes: &[u8], block_size: usize) -> Vec<u8> {
    assert!(block_size < 0x100, "only support block sizes up to 0x100");
    let bytes_len = bytes.len();
    let padding = (block_size - bytes_len % block_size) as u8;
    let padding_bytes = if padding != 0 { padding as usize } else { block_size };
    let mut padded = Vec::new();
    padded.extend(bytes);
    padded.extend(vec![padding as u8; padding_bytes]);
    padded
}

pub fn unpad(bytes: &[u8], block_size: usize) -> Result<Vec<u8>, &str> {
    assert!(block_size < 0x100, "only support block sizes up to 0x100");
    let len = bytes.len();
    if len % block_size != 0 {
        return Err("input length should be a multiple of block size to unpad");
    }
    let padding_value = bytes[len - 1] as usize;
    if padding_value > block_size {
        return Err("padding value was greater than block size");
    }
    for byte_n in 0..padding_value as usize {
        if bytes[len - byte_n - 1] != padding_value as u8 {
            return Err("padding value wasn't consistent");
        }
    }
    Ok(bytes[..len - padding_value].to_vec())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn pad_input_less_than_one_block() {
        let input = "YELLOW SUBMARINE".as_bytes();
        let expected = "YELLOW SUBMARINE\x04\x04\x04\x04".as_bytes();
        let output = pad(&input, 20);
        assert_eq!(output.as_slice(), expected);
    }

    #[test]
    fn pad_input_size_of_block() {
        let input = "YELLOW S".as_bytes();
        let expected = "YELLOW S\x08\x08\x08\x08\x08\x08\x08\x08".as_bytes();
        let output = pad(&input, 8);
        assert_eq!(output.as_slice(), expected);
    }

    #[test]
    fn pad_input_multiple_of_block() {
        let input = "YELLOW SUBMARINE".as_bytes();
        let expected = "YELLOW SUBMARINE\x08\x08\x08\x08\x08\x08\x08\x08".as_bytes();
        let output = pad(&input, 8);
        assert_eq!(output.as_slice(), expected);
    }

    #[test]
    fn pad_input_longer_than_block() {
        let input = "YELLOW SUBMARI".as_bytes();
        let expected = "YELLOW SUBMARI\x02\x02".as_bytes();
        let output = pad(&input, 8);
        assert_eq!(output.as_slice(), expected);
    }

    #[test]
    fn unpad_input_longer_than_block() {
        let input = "YELLOW SUBMARI\x02\x02".as_bytes();
        let output = unpad(input, 8);
        let expected = "YELLOW SUBMARI".as_bytes();
        assert_eq!(output.unwrap(), expected);
    }

    #[test]
    fn unpad_input_multiple_of_block() {
        let input = "YELLOW SUBMARINE\x08\x08\x08\x08\x08\x08\x08\x08".as_bytes();
        let expected = "YELLOW SUBMARINE".as_bytes();
        let output = unpad(&input, 8);
        assert_eq!(output.unwrap(), expected);
    }

    #[test]
    fn unpad_wrong_value() {
        let input = "ICE ICE BABY\x05\x05\x05\x05".as_bytes();
        if let Err(err) = unpad(&input, 16) {
            assert_eq!(err, "padding value wasn't consistent");
        } else {
            panic!("should have been an Err");
        }
    }

    #[test]
    fn unpad_not_multiple_of_blocksize() {
        let input = "ICE ICE BABY\x03\x03\x03".as_bytes();
        if let Err(err) = unpad(&input, 16) {
            assert_eq!(err, "input length should be a multiple of block size to unpad");
        } else {
            panic!("should have been an Err");
        }
    }

    #[test]
    fn unpad_weird_padding_value() {
        let input = "ICE ICE BABY\x33\x33\x33\x33".as_bytes();
        if let Err(err) = unpad(&input, 16) {
            assert_eq!(err, "padding value was greater than block size");
        } else {
            panic!("should have been an Err");
        }
    }
}
