use std::collections::HashSet;

pub struct Solution {}

impl Solution {
    pub fn partition_string(s: String) -> Vec<String> {
        let mut set = HashSet::new();
        let mut result = Vec::new();
        let mut i = 0;

        while i < s.len() {
            let mut found = false;
            for j in i..s.len() {
                if !set.contains(&s[i..=j]) {
                    set.insert(s[i..=j].to_string());
                    result.push(s[i..=j].to_string());
                    i = j + 1;
                    found = true;
                    break;
                }
            }
            if !found {
                break;
            }
        }

        result
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_3597() {
        assert_eq!(
            Solution::partition_string("abbccccd".to_string()),
            vec![
                "a".to_string(),
                "b".to_string(),
                "bc".to_string(),
                "c".to_string(),
                "cc".to_string(),
                "d".to_string()
            ]
        );
        assert_eq!(
            Solution::partition_string("aaaa".to_string()),
            vec!["a".to_string(), "aa".to_string()]
        );
    }
}
