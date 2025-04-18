use std::collections::HashMap;

pub struct Solution {}

impl Solution {
    pub fn shortest_substrings(arr: Vec<String>) -> Vec<String> {
        let mut count = HashMap::new();
        let mut substrings = vec![vec![]; arr.len()];
        let mut count2 = vec![HashMap::new(); arr.len()];
        for i in 0..arr.len() {
            for j in 0..arr[i].len() {
                for k in 0..=j {
                    *count.entry(&arr[i][k..=j]).or_insert(0) += 1;
                    *count2[i].entry(&arr[i][k..=j]).or_insert(0) += 1;
                    substrings[i].push(arr[i][k..=j].to_string());
                }
            }
        }
        (0..arr.len())
            .map(|i| {
                substrings[i].sort_unstable_by(|a, b| a.len().cmp(&b.len()).then(a.cmp(&b)));
                substrings[i]
                    .iter()
                    .find(|s| count[s.as_str()] == count2[i][s.as_str()])
                    .unwrap_or(&"".to_string())
                    .to_string()
            })
            .collect()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_3076() {
        assert_eq!(
            Solution::shortest_substrings(vec![
                "cab".to_string(),
                "ad".to_string(),
                "bad".to_string(),
                "c".to_string()
            ]),
            vec![
                "ab".to_string(),
                "".to_string(),
                "ba".to_string(),
                "".to_string()
            ]
        );
        assert_eq!(
            Solution::shortest_substrings(vec![
                "abc".to_string(),
                "bcd".to_string(),
                "abcd".to_string()
            ]),
            vec!["".to_string(), "".to_string(), "abcd".to_string()]
        );
    }
}
