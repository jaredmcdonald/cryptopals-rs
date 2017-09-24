fn parse_hex(hex: &str) -> Vec<u8> {
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

// http://fm4dd.com/programming/base64/base64_algorithm.htm
// https://stackoverflow.com/questions/10691186/how-do-you-convert-8-bit-bytes-to-6-bit-characters
// 🚨  note: does not implement padding
fn to_base64(bytes: Vec<u8>) -> String {
    let len = bytes.len();
    let mut chars = Vec::new(); // ugh, how to just do this functionally?
    for index in 0..len / 3 {
        let i = index * 3;
        let as_six_bit = [
            bytes[i] >> 2,
            ((bytes[i] & 0x3) << 4) | (bytes[i + 1] >> 4),
            ((bytes[i + 1] & 0xf) << 2) | (bytes[i + 2] >> 6),
            bytes[i + 2] & 0x3f
        ];

        for c in as_six_bit.iter().map(|n| ENCODING[*n as usize]) {
            chars.push(c);
        }
    }
    chars.into_iter().collect()
}

fn hex_to_base64(hex: &str) -> String {
    to_base64(parse_hex(hex))
}

fn test() {
    let output = "SSdtIGtpbGxpbmcgeW91ciBicmFpbiBsaWtlIGEgcG9pc29ub3VzIG11c2hyb29t";
    let input = "49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d";
    let real = hex_to_base64(input);
    if output != real {
        println!("❌\nexp: {}\ngot: {}", output, real);
    } else {
        println!("✅");
    }
}

fn main() {
    test()
}

const ENCODING: [char; 64] = [
  'A',
  'B',
  'C',
  'D',
  'E',
  'F',
  'G',
  'H',
  'I',
  'J',
  'K',
  'L',
  'M',
  'N',
  'O',
  'P',
  'Q',
  'R',
  'S',
  'T',
  'U',
  'V',
  'W',
  'X',
  'Y',
  'Z',
  'a',
  'b',
  'c',
  'd',
  'e',
  'f',
  'g',
  'h',
  'i',
  'j',
  'k',
  'l',
  'm',
  'n',
  'o',
  'p',
  'q',
  'r',
  's',
  't',
  'u',
  'v',
  'w',
  'x',
  'y',
  'z',
  '0',
  '1',
  '2',
  '3',
  '4',
  '5',
  '6',
  '7',
  '8',
  '9',
  '+',
  '/'
];
