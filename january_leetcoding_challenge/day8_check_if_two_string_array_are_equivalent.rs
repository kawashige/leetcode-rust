pub struct Solution {}

impl Solution {
    pub fn array_strings_are_equal(word1: Vec<String>, word2: Vec<String>) -> bool {
        word1.join("") == word2.join("")
    }
}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn test_day8() {
        assert!(Solution::array_strings_are_equal(
            vec!["ab".to_string(), "c".to_string()],
            vec!["a".to_string(), "bc".to_string()]
        ));
        assert!(!Solution::array_strings_are_equal(
            vec!["a".to_string(), "cb".to_string()],
            vec!["ab".to_string(), "c".to_string()]
        ));
        assert!(Solution::array_strings_are_equal(
            vec!["abc".to_string(), "d".to_string(), "efgh".to_string()],
            vec!["abcdefgh".to_string()]
        ));
    }
}
