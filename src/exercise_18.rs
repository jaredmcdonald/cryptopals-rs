use base64::decode;
use aes::aes_ctr;

pub fn run_18() {
    let ciphertext = decode("L77na/nrFsKvynd6HzOoG7GHTLXsTVu9qvY/2syLXzhPweyyMTJULu/6/kXX0KSvoOLSFQ==").unwrap();
    let plaintext = aes_ctr(&ciphertext, b"YELLOW SUBMARINE");
    println!("{}", plaintext.iter().map(|b| *b as char).collect::<String>());
}
