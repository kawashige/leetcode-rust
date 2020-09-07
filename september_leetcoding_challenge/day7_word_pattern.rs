pub struct Solution {}

impl Solution {
    pub fn word_pattern(pattern: String, str: String) -> bool {
        use std::collections::HashMap;
        let mut memo_p = HashMap::new();
        let mut memo_s = HashMap::new();
        let patterns = pattern.chars().collect::<Vec<char>>();
        let strings = str
            .split_whitespace()
            .map(|x| x.to_string())
            .collect::<Vec<String>>();
        if patterns.len() != strings.len() {
            return false;
        }
        for i in 0..patterns.len() {
            match (memo_p.get(&patterns[i]), memo_s.get(&strings[i])) {
                (Some(m_p), Some(m_s)) => {
                    if m_p != &strings[i] || m_s != &patterns[i] {
                        return false;
                    }
                }
                (None, None) => {
                    memo_s.insert(strings[i].clone(), patterns[i]);
                    memo_p.insert(patterns[i], strings[i].clone());
                }
                (_, _) => {
                    return false;
                }
            }
        }
        true
    }
}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn test_day7() {
        assert!(Solution::word_pattern(
            "abba".to_string(),
            "dog cat cat dog".to_string()
        ));
        assert!(!Solution::word_pattern(
            "abba".to_string(),
            "dog cat cat fish".to_string()
        ));
        assert!(!Solution::word_pattern(
            "aaaa".to_string(),
            "dog cat cat dog".to_string()
        ));
        assert!(!Solution::word_pattern(
            "abba".to_string(),
            "dog dog dog dog".to_string()
        ));
        assert!(!Solution::word_pattern(
            "aaa".to_string(),
            "dog dog dog dog".to_string()
        ));
    }
}
