pub struct Solution {}

impl Solution {
    pub fn letter_value(word: &str) -> usize {
        word.as_bytes()
            .iter()
            .fold(0, |acc, b| acc * 10 + (b - b'a') as usize)
    }

    pub fn is_sum_equal(first_word: String, second_word: String, target_word: String) -> bool {
        Self::letter_value(&first_word) + Self::letter_value(&second_word)
            == Self::letter_value(&target_word)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_1880() {
        assert!(Solution::is_sum_equal(
            "acb".to_string(),
            "cba".to_string(),
            "cdb".to_string()
        ));
        assert!(!Solution::is_sum_equal(
            "aaa".to_string(),
            "a".to_string(),
            "aab".to_string()
        ));
        assert!(Solution::is_sum_equal(
            "aaa".to_string(),
            "a".to_string(),
            "aaaa".to_string()
        ));
    }
}
