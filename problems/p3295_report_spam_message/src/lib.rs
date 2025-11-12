use std::collections::HashSet;

pub struct Solution {}

impl Solution {
    pub fn report_spam(message: Vec<String>, banned_words: Vec<String>) -> bool {
        let set: HashSet<String> = HashSet::from_iter(banned_words.into_iter());
        let mut count = 0;
        for m in message {
            if set.contains(&m) {
                count += 1;
                if count == 2 {
                    return true;
                }
            }
        }
        false
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_3295() {
        assert!(Solution::report_spam(
            vec![
                "hello".to_string(),
                "world".to_string(),
                "leetcode".to_string()
            ],
            vec!["world".to_string(), "hello".to_string()]
        ));
        assert!(!Solution::report_spam(
            vec![
                "hello".to_string(),
                "programming".to_string(),
                "fun".to_string()
            ],
            vec![
                "world".to_string(),
                "programming".to_string(),
                "leetcode".to_string()
            ]
        ));
    }
}
