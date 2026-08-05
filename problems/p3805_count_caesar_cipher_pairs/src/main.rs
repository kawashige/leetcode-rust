use std::collections::HashMap;

pub struct Solution {}

impl Solution {
    pub fn count_pairs(words: Vec<String>) -> i64 {
        let mut count = HashMap::new();
        let mut result = 0;

        for i in 0..words.len() {
            let d = words[i].as_bytes()[0] - b'a';
            let shifted = words[i]
                .as_bytes()
                .iter()
                .map(|b| (((b + 26 - d) % 26) + b'a') as char)
                .collect::<String>();
            result += count.get(&shifted).unwrap_or(&0);
            *count.entry(shifted).or_insert(0) += 1;
        }

        result
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_3805() {
        assert_eq!(
            Solution::count_pairs(vec!["fusion".to_string(), "layout".to_string()]),
            1
        );
        assert_eq!(
            Solution::count_pairs(vec![
                "ab".to_string(),
                "aa".to_string(),
                "za".to_string(),
                "aa".to_string()
            ]),
            2
        );
    }
}

fn main() {}
