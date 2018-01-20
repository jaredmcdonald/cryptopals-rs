use read_file::strings_from_filename;
use utils::{flatten, decode_base64_lines, random_bytes};
use aes::{decrypt_aes_ecb, aes_ctr, BLOCK_SIZE};

pub fn run_25() {
    // same as 1.7
    let ecb_key = "YELLOW SUBMARINE".as_bytes();
    let ecb_ciphertext = flatten(&decode_base64_lines(&strings_from_filename("25.txt")));
    let plaintext = decrypt_aes_ecb(&ecb_ciphertext, ecb_key);

    let key = random_bytes(BLOCK_SIZE);
    let nonce = 0; // don't think it would make a difference if this were randomized or not
    let ctr_ciphertext = aes_ctr(&plaintext, &key, nonce);

    let edit = |ciphertext: &[u8], offset: usize, new_text: &[u8]| -> Vec<u8> {
        let mut decrypted = aes_ctr(&ciphertext, &key, nonce);
        for i in offset..offset + new_text.len() {
            decrypted[i] = new_text[i - offset];
        }
        aes_ctr(&decrypted, &key, nonce)
    };

    let mut cracked_plaintext = vec![];
    for (index, _) in ctr_ciphertext.iter().enumerate() {
        for potential_byte_u16 in 0..256u16 {
            // need to do this until inclusive ranges land
            // https://github.com/rust-lang/rust/issues/28237
            let potential_byte = potential_byte_u16 as u8;

            let edited = edit(&ctr_ciphertext[..index + 1], index, &[potential_byte]);
            if edited[index] == ctr_ciphertext[index] {
                cracked_plaintext.push(potential_byte);
                break;
            }
        }
    }

    println!("{}", cracked_plaintext.iter().map(|b| *b as char).collect::<String>());
}
