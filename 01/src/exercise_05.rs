use ascii::ascii_to_bytes;
use hex::as_hex;
use repeating_key_xor::repeating_key_xor;

pub fn run_05() {
    let input = ascii_to_bytes("Burning 'em, if you ain't quick and nimble\nI go crazy when I hear a cymbal");
    let key = ascii_to_bytes("ICE");
    let expected_output = "0b3637272a2b2e63622c2e69692a23693a2a3c6324202d623d63343c2a26226324272765272a282b2f20430a652e2c652a3124333a653e2b2027630c692b20283165286326302e27282f";
    let hex_output = as_hex(&repeating_key_xor(&input, &key));
    println!("{}\nmatches expectation: {}", hex_output, hex_output == expected_output);
}
