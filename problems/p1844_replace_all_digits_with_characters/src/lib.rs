pub struct Solution {}

impl Solution {
    pub fn replace_digits(s: String) -> String {
        (0..s.len())
            .map(|i| {
                if i % 2 == 0 {
                    s.as_bytes()[i] as char
                } else {
                    (s.as_bytes()[i - 1] + s.as_bytes()[i] - b'0') as char
                }
            })
            .collect()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_1844() {
        assert_eq!(
            Solution::replace_digits("a1c1e1".to_string()),
            "abcdef".to_string()
        );
        assert_eq!(
            Solution::replace_digits("a1b2c3d4e".to_string()),
            "abbdcfdhe".to_string()
        );
    }
}
