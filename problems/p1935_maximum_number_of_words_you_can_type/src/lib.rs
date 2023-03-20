pub struct Solution {}

impl Solution {
    pub fn characters(s: &str) -> usize {
        s.as_bytes().iter().fold(0, |acc, b| acc | 1 << (b - b'a'))
    }

    pub fn can_be_typed_words(text: String, broken_letters: String) -> i32 {
        let broken = Self::characters(&broken_letters);
        text.split_ascii_whitespace()
            .filter(|s| Self::characters(s) & broken == 0)
            .count() as i32
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_1935() {
        assert_eq!(
            Solution::can_be_typed_words("hello world".to_string(), "ad".to_string()),
            1
        );
        assert_eq!(
            Solution::can_be_typed_words("leet code".to_string(), "lt".to_string()),
            1
        );
        assert_eq!(
            Solution::can_be_typed_words("leet cpde".to_string(), "e".to_string()),
            0
        );
    }
}
