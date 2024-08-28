pub struct Solution {}

impl Solution {
    pub fn make_smallest_palindrome(s: String) -> String {
        let mut chars = s.chars().collect::<Vec<_>>();
        let l = chars.len();
        for i in 0..chars.len() / 2 {
            if chars[i] != chars[l - 1 - i] {
                let c = chars[i].min(chars[l - 1 - i]);
                chars[i] = c;
                chars[l - 1 - i] = c;
            }
        }
        chars.into_iter().collect()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_2697() {
        assert_eq!(
            Solution::make_smallest_palindrome("egcfe".to_string()),
            "efcfe".to_string()
        );
        assert_eq!(
            Solution::make_smallest_palindrome("abcd".to_string()),
            "abba".to_string()
        );
        assert_eq!(
            Solution::make_smallest_palindrome("seven".to_string()),
            "neven".to_string()
        );
    }
}
