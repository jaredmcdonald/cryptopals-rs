use ascii::bytes_to_ascii_string;
use std::collections::HashMap;
use aes_oracles::random_key;
use aes::{encrypt_aes_ecb, decrypt_aes_ecb};

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

struct OracleResult {
    ciphertext: Vec<u8>,
    key: Vec<u8>,
}

fn encryption_oracle(email: &str) -> OracleResult {
    let key = random_key();
    let encoded_profile = encode(profile_for(email));
    let ciphertext = encrypt_aes_ecb(encoded_profile.as_bytes(), &key);
    OracleResult { ciphertext, key: key.to_vec() }
}

pub fn run_13() {
    let OracleResult { ciphertext, key } = encryption_oracle("foobar@bar.com");
    let decrypted = decrypt_aes_ecb(&ciphertext, &key);
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
        let mut input = HashMap::new();
        input.insert("foo", "bar".to_string());
        input.insert("baz", "qux".to_string());
        input.insert("zap", "zazzle".to_string());
        let output = encode(input);
        assert!(
            // no order, need to check all possibilities
            output == "foo=bar&baz=qux&zap=zazzle" ||
            output == "foo=bar&zap=zazzle&baz=qux" ||
            output == "baz=qux&foo=bar&zap=zazzle" ||
            output == "baz=qux&zap=zazzle&foo=bar" ||
            output == "zap=zazzle&baz=qux&foo=bar" ||
            output == "zap=zazzle&foo=bar&baz=qux"
        );
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
