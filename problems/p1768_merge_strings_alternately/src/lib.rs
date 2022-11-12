pub struct Solution {}

impl Solution {
    pub fn merge_alternately(word1: String, word2: String) -> String {
        let mut s = String::new();

        for i in 0..word1.len().max(word2.len()) {
            if i < word1.len() {
                s.push(word1.as_bytes()[i] as char);
            }
            if i < word2.len() {
                s.push(word2.as_bytes()[i] as char);
            }
        }

        s
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_1768() {
        assert_eq!(
            Solution::merge_alternately("abc".to_string(), "pqr".to_string()),
            "apbqcr".to_string()
        );
        assert_eq!(
            Solution::merge_alternately("ab".to_string(), "pqrs".to_string()),
            "apbqrs".to_string()
        );
        assert_eq!(
            Solution::merge_alternately("abcd".to_string(), "pq".to_string()),
            "apbqcd".to_string()
        );
    }
}
