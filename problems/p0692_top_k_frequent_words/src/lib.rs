use std::collections::HashMap;
pub struct Solution {}

impl Solution {
    pub fn top_k_frequent(words: Vec<String>, k: i32) -> Vec<String> {
        let mut v = words
            .into_iter()
            .fold(HashMap::new(), |mut map, w| {
                *map.entry(w).or_insert(0) += 1;
                map
            })
            .into_iter()
            .map(|(k, v)| (v, k))
            .collect::<Vec<(usize, String)>>();
        v.sort_unstable_by(|a, b| b.0.cmp(&a.0).then(a.1.cmp(&b.1)));
        v.into_iter().take(k as usize).map(|(_, k)| k).collect()
    }
}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn test_0692() {
        assert_eq!(
            Solution::top_k_frequent(
                vec![
                    "i".to_string(),
                    "love".to_string(),
                    "leetcode".to_string(),
                    "i".to_string(),
                    "love".to_string(),
                    "coding".to_string()
                ],
                2
            ),
            vec!["i".to_string(), "love".to_string()]
        );
        assert_eq!(
            Solution::top_k_frequent(
                vec![
                    "the".to_string(),
                    "day".to_string(),
                    "is".to_string(),
                    "sunny".to_string(),
                    "the".to_string(),
                    "the".to_string(),
                    "the".to_string(),
                    "sunny".to_string(),
                    "is".to_string(),
                    "is".to_string()
                ],
                4
            ),
            vec![
                "the".to_string(),
                "is".to_string(),
                "sunny".to_string(),
                "day".to_string()
            ]
        );
    }
}
