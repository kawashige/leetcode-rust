pub struct Solution {}

impl Solution {
    pub fn first_palindrome(words: Vec<String>) -> String {
        words
            .into_iter()
            .find(|s| s == &s.chars().rev().collect::<String>())
            .unwrap_or(String::new())
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_2108() {
        assert_eq!(
            Solution::first_palindrome(vec![
                "abc".to_string(),
                "car".to_string(),
                "ada".to_string(),
                "racecar".to_string(),
                "cool".to_string()
            ]),
            "ada".to_string()
        );
        assert_eq!(
            Solution::first_palindrome(vec!["notapalindrome".to_string(), "racecar".to_string()]),
            "racecar".to_string()
        );
        assert_eq!(
            Solution::first_palindrome(vec!["def".to_string(), "ghi".to_string()]),
            "".to_string()
        );
    }
}
