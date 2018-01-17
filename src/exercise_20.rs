use read_file::strings_from_filename;
use base64::decode;
use utils::xor_buffers;
use break_aes_ctr::break_fixed_nonce_aes_ctr;

pub fn run_20() {
    let b64_lines = strings_from_filename("20.txt");
    let ciphertexts = b64_lines.iter().map(|l| decode(l).unwrap()).collect::<Vec<_>>();
    let shortest_len = ciphertexts.iter().map(|ct| ct.len()).min().unwrap();
    let truncated_ciphertexts = ciphertexts.iter().map(|ct| &ct[..shortest_len - 1]).collect::<Vec<_>>();

    let keystream = break_fixed_nonce_aes_ctr(&truncated_ciphertexts);

    for ciphertext in &truncated_ciphertexts {
        println!("{}",
            xor_buffers(&ciphertext, &keystream)
                .iter()
                .map(|b| *b as char)
                .collect::<String>());
    }
}
