pub struct Solution {}

impl Solution {
    pub fn is_acronym(words: Vec<String>, s: String) -> bool {
        words.len() == s.len()
            && (0..words.len()).all(|i| words[i].as_bytes()[0] == s.as_bytes()[i])
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_2828() {
        assert!(Solution::is_acronym(
            vec![
                "alice".to_string(),
                "bob".to_string(),
                "charlie".to_string()
            ],
            "abc".to_string()
        ));
        assert!(!Solution::is_acronym(
            vec!["an".to_string(), "apple".to_string()],
            "a".to_string()
        ));
        assert!(Solution::is_acronym(
            vec![
                "never".to_string(),
                "gonna".to_string(),
                "give".to_string(),
                "up".to_string(),
                "on".to_string(),
                "you".to_string()
            ],
            "ngguoy".to_string()
        ))
    }
}
