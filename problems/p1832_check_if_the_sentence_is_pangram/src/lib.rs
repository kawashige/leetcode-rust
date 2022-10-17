pub struct Solution {}

impl Solution {
    pub fn check_if_pangram(sentence: String) -> bool {
        sentence
            .as_bytes()
            .iter()
            .fold(0, |acc, b| acc | 1 << b - b'a')
            == 2_usize.pow(26) - 1
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_1832() {
        assert!(Solution::check_if_pangram(
            "thequickbrownfoxjumpsoverthelazydog".to_string()
        ));
        assert!(!Solution::check_if_pangram("leetcode".to_string()));
    }
}
