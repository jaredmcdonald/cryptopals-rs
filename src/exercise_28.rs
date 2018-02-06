use sha1::Sha1;

fn keyed_mac(key: &[u8], msg: &[u8]) -> [u8; 20] {
    let mut m = Sha1::new();
    m.update(key);
    m.update(msg);
    m.digest().bytes()
}

pub fn run_28() {
    let key = b"secret_key";
    let msg = b"this is a message";

    assert_eq!(keyed_mac(key, msg), keyed_mac(key, msg));
    assert_ne!(keyed_mac(key, msg), keyed_mac(b"secret-key", msg));
    assert_ne!(keyed_mac(key, msg), keyed_mac(key, b"this is b message"));
    println!("OK");
}
