use hex::{parse_hex, as_hex};
use utils::xor_buffers;

fn fixed_xor(input: &str, xor_by: &str) -> String {
   let parsed_input = parse_hex(input);
   let parsed_xor_by = parse_hex(xor_by);
   as_hex(&xor_buffers(&parsed_input, &parsed_xor_by))
}

fn test() {
    let input = "1c0111001f010100061a024b53535009181c";
    let xor_by = "686974207468652062756c6c277320657965";
    let output = "746865206b696420646f6e277420706c6179";
    let real = fixed_xor(input, xor_by);
    if output != real {
        println!("❌\nexp: {}\ngot: {}", output, real);
    } else {
        println!('✅');
    }
}

pub fn run_02() {
    test();
}
