pub struct Solution {}

impl Solution {
    pub fn make_equal(words: Vec<String>) -> bool {
        let words_len = words.len();
        words
            .into_iter()
            .fold([0; 26], |mut count, word| {
                for b in word.as_bytes() {
                    count[(b - b'a') as usize] += 1;
                }
                count
            })
            .into_iter()
            .all(|count| count == 0 || count % words_len == 0)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_1897() {
        assert!(Solution::make_equal(vec![
            "abc".to_string(),
            "aabc".to_string(),
            "bc".to_string()
        ]));
        assert!(!Solution::make_equal(vec![
            "ab".to_string(),
            "a".to_string()
        ]));
    }
}
