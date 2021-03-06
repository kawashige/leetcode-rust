use std::collections::HashSet;
pub struct Solution {}

impl Solution {
    pub fn minimum_length_encoding(mut words: Vec<String>) -> i32 {
        let mut set = HashSet::new();
        words.sort_unstable_by_key(|w| -(w.len() as i32));
        let mut result = 0;
        for w in words {
            if set.contains(&w) {
                continue;
            }
            result += w.len() + 1;
            for i in 0..w.len() {
                set.insert(w[i..].to_string());
            }
        }
        result as i32
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_day06() {
        assert_eq!(
            Solution::minimum_length_encoding(vec![
                "time".to_string(),
                "me".to_string(),
                "bell".to_string()
            ]),
            10
        );
        assert_eq!(Solution::minimum_length_encoding(vec!["t".to_string()]), 2);
    }
}
