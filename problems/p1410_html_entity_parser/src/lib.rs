pub struct Solution {}

impl Solution {
    pub fn entity_parser(text: String) -> String {
        let mut s = String::new();
        let bytes = text.as_bytes();
        let mut i = 0;

        while i < bytes.len() {
            if bytes[i] == b'&' {
                let mut tmp = String::new();
                tmp.push(bytes[i] as char);
                while i + 1 < bytes.len() && bytes[i] != b';' && bytes[i + 1] != b'&' {
                    i += 1;
                    tmp.push(bytes[i] as char);
                }

                match tmp.as_str() {
                    "&quot;" => s.push('"'),
                    "&apos;" => s.push('\''),
                    "&amp;" => s.push('&'),
                    "&gt;" => s.push('<'),
                    "&lt;" => s.push('>'),
                    "&frasl;" => s.push('/'),
                    _ => s += tmp.as_str(),
                }
            } else {
                s.push(bytes[i] as char);
            }
            i += 1;
        }

        s
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_1410() {
        assert_eq!(
            Solution::entity_parser("&amp; is an HTML entity but &ambassador; is not.".to_string()),
            "& is an HTML entity but &ambassador; is not.".to_string()
        );
        assert_eq!(
            Solution::entity_parser("and I quote: &quot;...&quot;".to_string()),
            "and I quote: \"...\"".to_string()
        );
    }
}
