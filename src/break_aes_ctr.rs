use english::score_bytes;

pub fn break_fixed_nonce_aes_ctr(ciphertexts: &[&[u8]]) -> Vec<u8> { // return the keystream
    let longest_ciphertext = ciphertexts.iter().map(|ct| ct.len()).max().unwrap();

    let mut keystream = vec![];
    for ct_index in 0..longest_ciphertext {
        let mut ciphertext_bytes_at_index = vec![];
        for ciphertext in ciphertexts {
            if let Some(ct_byte) = ciphertext.iter().nth(ct_index) {
                ciphertext_bytes_at_index.push(*ct_byte);
            }
        }

        let mut best_score = 0f64;
        let mut best_byte = 0u8;

        for xor_byte in 0..0xff {
            let xored = ciphertext_bytes_at_index.iter().map(|b| b ^ xor_byte).collect::<Vec<_>>();
            let score = score_bytes(&xored);
            if score > best_score {
                best_score = score;
                best_byte = xor_byte;
            }
        }
        keystream.push(best_byte);
    }
    keystream
}
