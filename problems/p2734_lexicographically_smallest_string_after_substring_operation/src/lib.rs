pub struct Solution {}

impl Solution {
    pub fn preceeding(b: u8) -> char {
        (((b - b'a') + 26 - 1) % 26 + b'a') as char
    }

    pub fn smallest_string(s: String) -> String {
        if s.len() == 1 {
            return s.as_bytes().iter().map(|b| Self::preceeding(*b)).collect();
        }

        if let Some(i) = (0..s.len()).find(|i| s.as_bytes()[*i] != b'a') {
            let j = (i + 1..s.len())
                .find(|i| s.as_bytes()[*i] == b'a')
                .unwrap_or(s.len());
            (0..s.len())
                .map(|k| {
                    if (i..j).contains(&k) {
                        Self::preceeding(s.as_bytes()[k])
                    } else {
                        s.as_bytes()[k] as char
                    }
                })
                .collect()
        } else {
            s[..s.len() - 1].to_string() + "z"
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_2734() {
        assert_eq!(
            Solution::smallest_string("cbabc".to_string()),
            "baabc".to_string()
        );
        assert_eq!(
            Solution::smallest_string("aa".to_string()),
            "az".to_string()
        );
        assert_eq!(
            Solution::smallest_string("acbbc".to_string()),
            "abaab".to_string()
        );
        assert_eq!(
            Solution::smallest_string("leetcode".to_string()),
            "kddsbncd".to_string()
        );
    }
}
