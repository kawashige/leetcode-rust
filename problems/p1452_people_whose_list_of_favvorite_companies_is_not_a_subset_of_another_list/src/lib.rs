use std::collections::HashMap;

pub struct Solution {}

impl Solution {
    pub fn people_indexes(favorite_companies: Vec<Vec<String>>) -> Vec<i32> {
        let map = favorite_companies.iter().enumerate().fold(
            HashMap::new(),
            |mut map, (i, companies)| {
                for company in companies {
                    *map.entry(company).or_insert(0_u128) |= 1 << i;
                }
                map
            },
        );

        favorite_companies
            .iter()
            .enumerate()
            .filter_map(|(i, companies)| {
                if companies
                    .into_iter()
                    .fold(!0, |acc, company| acc & map[&company])
                    == 1 << i
                {
                    Some(i as i32)
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
    fn test_1452() {
        assert_eq!(
            Solution::people_indexes(vec![
                vec![
                    "leetcode".to_string(),
                    "google".to_string(),
                    "facebook".to_string()
                ],
                vec!["google".to_string(), "microsoft".to_string()],
                vec!["google".to_string(), "facebook".to_string()],
                vec!["google".to_string()],
                vec!["amazon".to_string()]
            ]),
            vec![0, 1, 4]
        );
        assert_eq!(
            Solution::people_indexes(vec![
                vec![
                    "leetcode".to_string(),
                    "google".to_string(),
                    "facebook".to_string()
                ],
                vec!["leetcode".to_string(), "amazon".to_string()],
                vec!["facebook".to_string(), "google".to_string()]
            ]),
            vec![0, 1]
        );
        assert_eq!(
            Solution::people_indexes(vec![
                vec!["leetcode".to_string()],
                vec!["google".to_string()],
                vec!["facebook".to_string()],
                vec!["amazon".to_string()]
            ]),
            vec![0, 1, 2, 3]
        );
    }
}
