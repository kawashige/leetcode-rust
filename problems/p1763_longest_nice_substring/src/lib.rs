pub struct Solution {}

impl Solution {
    pub fn longest_nice_substring(s: String) -> String {
        for l in (2..=s.len()).rev() {
            for i in 0..=s.len() - l {
                let mut upper = 0;
                let mut lower = 0;
                for j in i..i + l {
                    if s.as_bytes()[j].is_ascii_lowercase() {
                        lower |= 1 << s.as_bytes()[j] - b'a';
                    } else {
                        upper |= 1 << s.as_bytes()[j] - b'A';
                    }
                }

                if upper ^ lower == 0 {
                    return s[i..i + l].to_string();
                }
            }
        }

        Default::default()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_1763() {
        assert_eq!(
            Solution::longest_nice_substring("YazaAay".to_string()),
            "aAa".to_string()
        );
        assert_eq!(
            Solution::longest_nice_substring("Bb".to_string()),
            "Bb".to_string()
        );
        assert_eq!(
            Solution::longest_nice_substring("c".to_string()),
            "".to_string()
        );
    }
}
