use std::collections::HashMap;

pub struct Solution {}

impl Solution {
    pub fn uncommon_from_sentences(s1: String, s2: String) -> Vec<String> {
        s1.split_ascii_whitespace()
            .chain(s2.split_ascii_whitespace())
            .fold(HashMap::new(), |mut words, word| {
                *words.entry(word.to_string()).or_insert(0) += 1;
                words
            })
            .into_iter()
            .filter(|(_, c)| c == &1)
            .map(|(word, _)| word)
            .collect()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_0884() {
        assert_eq!(
            Solution::uncommon_from_sentences(
                "this apple is sweet".to_string(),
                "this apple is sour".to_string()
            ),
            vec!["sweet".to_string(), "sour".to_string()]
        );

        assert_eq!(
            Solution::uncommon_from_sentences("apple apple".to_string(), "banana".to_string()),
            vec!["banana".to_string()]
        );
    }
}
