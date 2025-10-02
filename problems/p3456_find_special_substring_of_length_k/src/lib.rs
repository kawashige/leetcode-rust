pub struct Solution {}

impl Solution {
    pub fn has_special_substring(s: String, k: i32) -> bool {
        let mut count = 0;

        for i in 0..s.len() {
            if 0 < i && s.as_bytes()[i - 1] == s.as_bytes()[i] {
                count += 1;
            } else {
                count = 1;
            }
            if count == k && (i == s.len() - 1 || s.as_bytes()[i] != s.as_bytes()[i + 1]) {
                return true;
            }
        }

        false
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_3456() {
        assert!(Solution::has_special_substring("aaabaaa".to_string(), 3));
        assert!(!Solution::has_special_substring("abc".to_string(), 2));
    }
}
