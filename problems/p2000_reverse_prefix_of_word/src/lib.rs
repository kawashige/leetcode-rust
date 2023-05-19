pub struct Solution {}

impl Solution {
    pub fn reverse_prefix(word: String, ch: char) -> String {
        if let Some(i) = word.chars().position(|c| c == ch) {
            word[..=i]
                .chars()
                .rev()
                .chain(word[i + 1..].chars())
                .collect()
        } else {
            word
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_2000() {
        assert_eq!(
            Solution::reverse_prefix("abcdefd".to_string(), 'd'),
            "dcbaefd".to_string()
        );
        assert_eq!(
            Solution::reverse_prefix("xyxzxe".to_string(), 'z'),
            "zxyxxe".to_string()
        );
        assert_eq!(
            Solution::reverse_prefix("abcd".to_string(), 'z'),
            "abcd".to_string()
        );
    }
}
