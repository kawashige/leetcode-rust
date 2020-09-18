pub struct Solution {}

impl Solution {
    pub fn word_break(s: String, word_dict: Vec<String>) -> bool {
        use std::collections::HashSet;

        fn recurse(s: &str, word_dict: &[String], memo: &mut HashSet<usize>) -> bool {
            if s.len() == 0 {
                return true;
            }
            if memo.contains(&s.len()) {
                return false;
            }
            for w in word_dict {
                if s.starts_with(w) {
                    if recurse(&s[w.len()..], word_dict, memo) {
                        return true;
                    }
                    memo.insert(s.len() - w.len());
                }
            }
            false
        }
        recurse(&s, &word_dict, &mut HashSet::new())
    }
}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn test_0139() {
        assert!(Solution::word_break(
            "leetcode".to_string(),
            vec!["leet".to_string(), "code".to_string()]
        ));
        assert!(Solution::word_break(
            "applepenapple".to_string(),
            vec!["apple".to_string(), "pen".to_string()]
        ));
        assert!(!Solution::word_break(
            "catsandog".to_string(),
            vec![
                "cats".to_string(),
                "sand".to_string(),
                "and".to_string(),
                "cat".to_string()
            ]
        ));

        assert!(!Solution::word_break(
            "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaab".to_string(),
            vec![
                "a".to_string(),
                "aa".to_string(),
                "aaa".to_string(),
                "aaaa".to_string(),
                "aaaaa".to_string(),
                "aaaaaa".to_string(),
                "aaaaaaa".to_string(),
                "aaaaaaaa".to_string(),
                "aaaaaaaaa".to_string(),
                "aaaaaaaaaa".to_string()
            ]
        ));
    }
}
