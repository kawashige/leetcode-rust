use std::collections::VecDeque;

pub struct Solution {}

impl Solution {
    pub fn are_sentences_similar(sentence1: String, sentence2: String) -> bool {
        let (short, long) = if sentence1.len() < sentence2.len() {
            (sentence1, sentence2)
        } else {
            (sentence2, sentence1)
        };

        let mut short_words = short.split_whitespace().collect::<VecDeque<_>>();
        let mut long_words = long.split_whitespace().collect::<VecDeque<_>>();

        while !short_words.is_empty() && short_words.front() == long_words.front() {
            short_words.pop_front();
            long_words.pop_front();
        }
        while !short_words.is_empty() && short_words.back() == long_words.back() {
            short_words.pop_back();
            long_words.pop_back();
        }

        short_words.is_empty()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_1813() {
        assert!(Solution::are_sentences_similar(
            "My name is Haley".to_string(),
            "My Haley".to_string()
        ));
        assert!(!Solution::are_sentences_similar(
            "of".to_string(),
            "A lot of words".to_string()
        ));
        assert!(Solution::are_sentences_similar(
            "Eating right now".to_string(),
            "Eating".to_string()
        ));
    }
}
