use rand::random;
use utils::{random_bytes, xor_buffers};
use aes::{aes_ctr, BLOCK_SIZE};
use ascii::bytes_to_ascii_string;
use exercise_16::generate_profile;

fn generate_encrypted_profile(content: &[u8], key: &[u8], nonce: u64) -> Vec<u8> {
    aes_ctr(&generate_profile(content), key, nonce)
}

pub fn run_26() {
    let key = random_bytes(BLOCK_SIZE);
    let nonce = random::<u64>();

    let payload = b"ha;admin=true;ha";
    let input = vec![0x0; BLOCK_SIZE];
    let ciphertext = generate_encrypted_profile(&input, &key, nonce);
    
    let mut manipulated_ciphertext = vec![];
    manipulated_ciphertext.extend(&ciphertext[..BLOCK_SIZE * 2]);
    manipulated_ciphertext.extend(
        xor_buffers(&ciphertext[BLOCK_SIZE * 2..BLOCK_SIZE * 3], payload)
    );
    manipulated_ciphertext.extend(&ciphertext[BLOCK_SIZE * 3..]);

    let decrypted_profile = aes_ctr(&manipulated_ciphertext, &key, nonce);
    let as_string = bytes_to_ascii_string(&decrypted_profile);

    println!("result:\n{}", as_string);
    println!("success? {}", as_string.contains(";admin=true;"));
}
