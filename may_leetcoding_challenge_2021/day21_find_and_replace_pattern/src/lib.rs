use std::collections::HashMap;

pub struct Solution {}

impl Solution {
    pub fn replace(s: &str) -> String {
        s.chars()
            .fold((HashMap::new(), String::new()), |(mut map, mut s), c| {
                let l = map.len();
                let r = map.entry(c).or_insert(('a' as u8 + l as u8) as char);
                s.push(*r);
                (map, s)
            })
            .1
    }

    pub fn find_and_replace_pattern(words: Vec<String>, pattern: String) -> Vec<String> {
        let pattern = Self::replace(&pattern);
        words
            .into_iter()
            .filter(|w| Self::replace(w) == pattern)
            .collect()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_day21() {
        assert_eq!(
            Solution::find_and_replace_pattern(
                vec![
                    "abc".to_string(),
                    "deq".to_string(),
                    "mee".to_string(),
                    "aqq".to_string(),
                    "dkd".to_string(),
                    "ccc".to_string()
                ],
                "abb".to_string()
            ),
            vec!["mee".to_string(), "aqq".to_string()]
        );

        assert_eq!(
            Solution::find_and_replace_pattern(
                vec!["a".to_string(), "b".to_string(), "c".to_string(),],
                "a".to_string()
            ),
            vec!["a".to_string(), "b".to_string(), "c".to_string()]
        );
    }
}
