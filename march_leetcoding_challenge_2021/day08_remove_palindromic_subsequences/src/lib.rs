pub struct Solution {}

impl Solution {
    pub fn remove_palindrome_sub(s: String) -> i32 {
        if s.is_empty() {
            0
        } else if s == s.chars().rev().collect::<String>() {
            1
        } else {
            2
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_day08() {
        assert_eq!(Solution::remove_palindrome_sub("ababa".to_string()), 1);
        assert_eq!(Solution::remove_palindrome_sub("abb".to_string()), 2);
        assert_eq!(Solution::remove_palindrome_sub("baabb".to_string()), 2);
        assert_eq!(Solution::remove_palindrome_sub("".to_string()), 0);
    }
}
