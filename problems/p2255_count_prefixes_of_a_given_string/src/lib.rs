pub struct Solution {}

impl Solution {
    pub fn count_prefixes(words: Vec<String>, s: String) -> i32 {
        words.into_iter().filter(|w| s.starts_with(w)).count() as i32
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_2255() {
        assert_eq!(
            Solution::count_prefixes(
                vec![
                    "a".to_string(),
                    "b".to_string(),
                    "c".to_string(),
                    "ab".to_string(),
                    "bc".to_string(),
                    "abc".to_string()
                ],
                "abc".to_string()
            ),
            3
        );
        assert_eq!(
            Solution::count_prefixes(vec!["a".to_string(), "a".to_string()], "aa".to_string()),
            2
        );
    }
}
