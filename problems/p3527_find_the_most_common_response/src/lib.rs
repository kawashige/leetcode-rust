use std::collections::{HashMap, HashSet};

pub struct Solution {}

impl Solution {
    pub fn find_common_response(responses: Vec<Vec<String>>) -> String {
        let mut count = responses
            .into_iter()
            .map(|r| {
                r.into_iter()
                    .collect::<HashSet<_>>()
                    .into_iter()
                    .collect::<Vec<_>>()
            })
            .flatten()
            .fold(HashMap::new(), |mut map, s| {
                *map.entry(s).or_insert(0) += 1;
                map
            })
            .into_iter()
            .collect::<Vec<_>>();
        count.sort_unstable_by(|a, b| b.1.cmp(&a.1).then(a.0.cmp(&b.0)));
        count[0].0.clone()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_3527() {
        assert_eq!(
            Solution::find_common_response(vec![
                vec![
                    "good".to_string(),
                    "ok".to_string(),
                    "good".to_string(),
                    "ok".to_string()
                ],
                vec![
                    "ok".to_string(),
                    "bad".to_string(),
                    "good".to_string(),
                    "ok".to_string(),
                    "ok".to_string()
                ],
                vec!["good".to_string()],
                vec!["bad".to_string()]
            ]),
            "good".to_string()
        );
        assert_eq!(
            Solution::find_common_response(vec![
                vec!["good".to_string(), "ok".to_string(), "good".to_string()],
                vec!["ok".to_string(), "bad".to_string()],
                vec!["bad".to_string(), "notsure".to_string()],
                vec!["great".to_string(), "good".to_string()]
            ]),
            "bad".to_string()
        );
    }
}
