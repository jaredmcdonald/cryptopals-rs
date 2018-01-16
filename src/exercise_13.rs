use ascii::bytes_to_ascii_string;
use std::collections::HashMap;
use utils::random_bytes;
use aes::{encrypt_aes_ecb_padded, decrypt_aes_ecb_padded, BLOCK_SIZE};

fn parse(s: &str) -> HashMap<&str, String> {
    let mut output = HashMap::new();
    for pair in s.split('&') {
        let parsed: Vec<&str> = pair.split('=').collect();
        output.insert(parsed[0], parsed[1].to_string());
    }
    output
}

fn encode(map: HashMap<&str, String>) -> String {
    let mut encoded_pairs = Vec::new();
    for (key, value) in map {
        encoded_pairs.push(format!("{}={}", key, value));
    }
    encoded_pairs.sort(); // give it some deterministic order, not sure if this is necessary
    encoded_pairs.join("&")
}

fn profile_for(email: &str) -> HashMap<&str, String> {
    let mut profile = HashMap::new();
    let sanitized_email = email.replace("=", "").replace("&", "");
    profile.insert("email", sanitized_email);
    profile.insert("uid", "10".to_string());
    profile.insert("role", "user".to_string());
    profile
}

fn encryption_oracle(email: &str, key: &[u8]) -> Vec<u8> {
    let encoded_profile = encode(profile_for(email));
    encrypt_aes_ecb_padded(encoded_profile.as_bytes(), key)
}

pub fn run_13() {
    let key = random_bytes(BLOCK_SIZE);

    // to be pasted over the real block
    let bogus_block = encrypt_aes_ecb_padded("role=admin&uid=1".as_bytes(), &key);

    // ends up encoded as 16 bytes: "email=f@bar.com&"
    // (one block, pushing everything else into the next block)
    let ciphertext = encryption_oracle("f@bar.com", &key);

    let mut manipulated_ciphertext = Vec::new();
    manipulated_ciphertext.extend(&ciphertext[..BLOCK_SIZE]);
    manipulated_ciphertext.extend(&bogus_block);

    let decrypted = decrypt_aes_ecb_padded(&manipulated_ciphertext, &key);
    let encoded_profile = bytes_to_ascii_string(&decrypted);
    let reassembled_profile = parse(&encoded_profile);

    println!("{:?}", reassembled_profile);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_string() {
        let input = "foo=bar&baz=qux&zap=zazzle";
        let mut expected = HashMap::new();
        expected.insert("foo", "bar".to_string());
        expected.insert("baz", "qux".to_string());
        expected.insert("zap", "zazzle".to_string());
        let output = parse(input);
        assert_eq!(output, expected);
    }

    #[test]
    fn encodes_profile() {
        // run 10x to show that we didn't just get lucky w/ key ordering,
        // it's actually deterministic (alpha sorted)
        for _ in 0..10 {
            let mut input = HashMap::new();
            input.insert("foo", "bar".to_string());
            input.insert("baz", "qux".to_string());
            input.insert("zap", "zazzle".to_string());
            let output = encode(input);
            assert_eq!(output, "baz=qux&foo=bar&zap=zazzle");
        }
    }

    #[test]
    fn creates_profile() {
        let email = "foo@bar.com";
        let mut expected = HashMap::new();
        expected.insert("email", email.to_string());
        expected.insert("role", "user".to_string());
        expected.insert("uid", "10".to_string());
        assert_eq!(profile_for(email), expected);
    }

    #[test]
    fn sanitizes_email() {
        let output = profile_for("sketchy_&email=@blargh=.&com");
        assert_eq!(output.get("email").unwrap(), "sketchy_email@blargh.com");
    }
}
