use std::collections::HashMap;

pub fn parse(s: &str) -> HashMap<&str, &str> {
    let mut output = HashMap::new();
    for pair in s.split('&') {
        let parsed: Vec<&str> = pair.split('=').collect();
        output.insert(parsed[0], parsed[1]);
    }
    output
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_string() {
        let input = "foo=bar&baz=qux&zap=zazzle";
        let mut expected = HashMap::new();
        expected.insert("foo", "bar");
        expected.insert("baz", "qux");
        expected.insert("zap", "zazzle");
        let output = parse(input);
        assert_eq!(output, expected);
    }
}
