pub struct Solution {}

use std::collections::HashMap;
impl Solution {
    pub fn find_restaurant(list1: Vec<String>, list2: Vec<String>) -> Vec<String> {
        let map: HashMap<String, usize> =
            list1.into_iter().enumerate().map(|(i, s)| (s, i)).collect();

        let mut result = Vec::new();
        let mut sum_index = std::usize::MAX;
        for i in 0..list2.len() {
            if let Some(i1) = map.get(&list2[i]) {
                if i1 + i < sum_index {
                    result = vec![list2[i].to_string()];
                    sum_index = i1 + i;
                } else if i1 + i == sum_index {
                    result.push(list2[i].to_string());
                }
            }
        }
        result
    }
}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn test_0599() {
        assert_eq!(
            Solution::find_restaurant(
                vec![
                    "Shogun".to_string(),
                    "Tapioca Express".to_string(),
                    "Burger King".to_string(),
                    "KFC".to_string()
                ],
                vec![
                    "Piatti".to_string(),
                    "The Grill at Torrey Pines".to_string(),
                    "Hungry Hunter Steakhouse".to_string(),
                    "Shogun".to_string()
                ]
            ),
            vec!["Shogun".to_string()]
        );
        assert_eq!(
            Solution::find_restaurant(
                vec![
                    "Shogun".to_string(),
                    "Tapioca Express".to_string(),
                    "Burger King".to_string(),
                    "KFC".to_string()
                ],
                vec![
                    "KFC".to_string(),
                    "Shogun".to_string(),
                    "Burger King".to_string()
                ]
            ),
            vec!["Shogun".to_string()]
        );
        assert_eq!(
            Solution::find_restaurant(
                vec![
                    "Shogun".to_string(),
                    "Tapioca Express".to_string(),
                    "Burger King".to_string(),
                    "KFC".to_string()
                ],
                vec![
                    "KFC".to_string(),
                    "Burger King".to_string(),
                    "Tapioca Express".to_string(),
                    "Shogun".to_string()
                ]
            ),
            vec![
                "KFC".to_string(),
                "Burger King".to_string(),
                "Tapioca Express".to_string(),
                "Shogun".to_string()
            ]
        );
        assert_eq!(
            Solution::find_restaurant(
                vec![
                    "Shogun".to_string(),
                    "Tapioca Express".to_string(),
                    "Burger King".to_string(),
                    "KFC".to_string()
                ],
                vec![
                    "KNN".to_string(),
                    "KFC".to_string(),
                    "Burger King".to_string(),
                    "Tapioca Express".to_string(),
                    "Shogun".to_string()
                ]
            ),
            vec![
                "KFC".to_string(),
                "Burger King".to_string(),
                "Tapioca Express".to_string(),
                "Shogun".to_string(),
            ]
        );
        assert_eq!(
            Solution::find_restaurant(vec!["KFC".to_string()], vec!["KFC".to_string()]),
            vec!["KFC".to_string()]
        );
    }
}
