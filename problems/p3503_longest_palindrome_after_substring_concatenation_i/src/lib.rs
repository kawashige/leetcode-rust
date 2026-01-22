pub struct Solution {}

impl Solution {
    pub fn is_palindrome(s: &str) -> bool {
        for i in 0..s.len() / 2 {
            if s.as_bytes()[i] != s.as_bytes()[s.len() - 1 - i] {
                return false;
            }
        }
        true
    }
    pub fn longest_palindrome(s: String, t: String) -> i32 {
        let mut result = 1;

        for i in 0..s.len() {
            for j in 0..=i {
                if Self::is_palindrome(&s[j..=i]) {
                    result = result.max(i - j + 1);
                }
                for k in 0..t.len() {
                    for l in 0..=k {
                        if Self::is_palindrome(&t[l..=k]) {
                            result = result.max(k - l + 1);
                        }
                        let str = format!("{}{}", &s[j..=i], &t[l..=k]);
                        if Self::is_palindrome(&str) {
                            result = result.max(str.len());
                        }
                    }
                }
            }
        }

        result as i32
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_3503() {
        assert_eq!(
            Solution::longest_palindrome("a".to_string(), "a".to_string()),
            2
        );
        assert_eq!(
            Solution::longest_palindrome("abc".to_string(), "def".to_string()),
            1
        );
        assert_eq!(
            Solution::longest_palindrome("b".to_string(), "aaaa".to_string()),
            4
        );
        assert_eq!(
            Solution::longest_palindrome("abcde".to_string(), "ecdba".to_string()),
            5
        );
    }
}
