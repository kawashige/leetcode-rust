use std::collections::HashSet;

pub struct Solution {}

impl Solution {
    pub fn to_number(s: &str) -> usize {
        s.as_bytes()
            .iter()
            .fold(0, |acc, b| acc | (1 << (b - b'a')))
    }

    pub fn word_count(start_words: Vec<String>, target_words: Vec<String>) -> i32 {
        let start_set = start_words
            .iter()
            .map(|s| Self::to_number(s))
            .collect::<HashSet<_>>();

        target_words
            .into_iter()
            .filter(|w| {
                let n = Self::to_number(&w);
                (0..26)
                    .filter(|i| 0 < n & 1 << i)
                    .any(|i| start_set.contains(&(n ^ 1 << i)))
            })
            .count() as i32
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_2135() {
        assert_eq!(
            Solution::word_count(
                vec!["ant".to_string(), "act".to_string(), "tack".to_string()],
                vec!["tack".to_string(), "act".to_string(), "acti".to_string()]
            ),
            2
        );
        assert_eq!(
            Solution::word_count(
                vec!["ab".to_string(), "a".to_string()],
                vec!["abc".to_string(), "abcd".to_string()]
            ),
            1
        );
    }
}
