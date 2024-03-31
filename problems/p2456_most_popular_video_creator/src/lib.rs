use std::collections::HashMap;

pub struct Solution {}

impl Solution {
    pub fn most_popular_creator(
        creators: Vec<String>,
        ids: Vec<String>,
        views: Vec<i32>,
    ) -> Vec<Vec<String>> {
        let mut max_total_views = 0;
        let mut map = HashMap::new();

        for i in 0..creators.len() {
            if !map.contains_key(&creators[i]) {
                map.insert(creators[i].clone(), (0, (-1, "")));
            }

            if let Some(creator) = map.get_mut(&creators[i]) {
                creator.0 += views[i] as i64;
                max_total_views = max_total_views.max(creator.0);
                if creator.1 .0 < views[i] || (creator.1 .0 == views[i] && creator.1 .1 > &ids[i]) {
                    creator.1 .0 = views[i];
                    creator.1 .1 = &ids[i]
                }
            }
        }

        map.into_iter()
            .filter_map(|(creator, (total_view, (view, id)))| {
                if max_total_views == total_view {
                    Some(vec![creator, id.to_string()])
                } else {
                    None
                }
            })
            .collect()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_2456() {
        assert_eq!(
            Solution::most_popular_creator(
                vec![
                    "alice".to_string(),
                    "bob".to_string(),
                    "alice".to_string(),
                    "chris".to_string()
                ],
                vec![
                    "one".to_string(),
                    "two".to_string(),
                    "three".to_string(),
                    "four".to_string()
                ],
                vec![5, 10, 5, 4]
            ),
            vec![
                vec!["alice".to_string(), "one".to_string()],
                vec!["bob".to_string(), "two".to_string()]
            ]
        );
        assert_eq!(
            Solution::most_popular_creator(
                vec![
                    "alice".to_string(),
                    "alice".to_string(),
                    "alice".to_string()
                ],
                vec!["a".to_string(), "b".to_string(), "c".to_string()],
                vec![1, 2, 2]
            ),
            vec![vec!["alice".to_string(), "b".to_string()]]
        );
    }
}
