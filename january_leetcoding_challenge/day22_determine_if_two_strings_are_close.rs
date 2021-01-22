pub struct Solution {}

use std::collections::HashSet;
impl Solution {
    pub fn close_strings(word1: String, word2: String) -> bool {
        fn occurrences(word: String) -> (HashSet<usize>, [usize; 26]) {
            let mut occurennce = [0; 26];
            for c in word.as_bytes() {
                occurennce[(*c - 0x61) as usize] += 1;
            }
            let indices = (0..26).filter(|i| 0 < occurennce[*i]).collect();
            occurennce.sort();
            (indices, occurennce)
        }

        if word1.len() != word2.len() {
            return false;
        }
        occurrences(word1) == occurrences(word2)
    }
}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn test_day22() {
        assert!(Solution::close_strings(
            "bca".to_string(),
            "abc".to_string()
        ));
        assert!(!Solution::close_strings("a".to_string(), "aa".to_string()));
        assert!(Solution::close_strings(
            "cabbba".to_string(),
            "abbccc".to_string()
        ));
        assert!(!Solution::close_strings(
            "cabbba".to_string(),
            "aabbss".to_string()
        ));
        assert!(!Solution::close_strings(
            "uau".to_string(),
            "ssx".to_string()
        ));
    }
}
