use std::collections::HashMap;

pub struct Solution {}

impl Solution {
    pub fn kth_distinct(arr: Vec<String>, k: i32) -> String {
        let counts = arr.iter().fold(HashMap::new(), |mut counts, s| {
            *counts.entry(s).or_insert(0) += 1;
            counts
        });
        let mut count = 0;
        for s in &arr {
            if counts[&s] == 1 {
                count += 1;
            }
            if count == k {
                return s.to_string();
            }
        }
        Default::default()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_2053() {
        assert_eq!(
            Solution::kth_distinct(
                vec![
                    "d".to_string(),
                    "b".to_string(),
                    "c".to_string(),
                    "b".to_string(),
                    "c".to_string(),
                    "a".to_string()
                ],
                2
            ),
            "a".to_string()
        );
        assert_eq!(
            Solution::kth_distinct(
                vec!["aaa".to_string(), "aa".to_string(), "a".to_string()],
                1
            ),
            "aaa".to_string()
        );
        assert_eq!(
            Solution::kth_distinct(vec!["a".to_string(), "b".to_string(), "a".to_string()], 3),
            "".to_string()
        );
    }
}
