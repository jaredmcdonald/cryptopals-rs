pub fn parse_hex(hex: &str) -> Vec<u8> {
    let mut bytes = Vec::new();
    for i in 0..(hex.len() / 2) {
        let index = i * 2;
        match u8::from_str_radix(&hex[index..(index + 2)], 16) {
            Ok(b) => bytes.push(b),
            Err(_) => println!("ah shit"),
        }
    }
    bytes
}

pub fn as_hex(bytes: &Vec<u8>) -> String {
    bytes.iter().map(|b| format!("{:x}", b)).collect()
}
