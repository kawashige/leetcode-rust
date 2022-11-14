pub struct Solution {}

impl Solution {
    pub fn count_matches(items: Vec<Vec<String>>, rule_key: String, rule_value: String) -> i32 {
        let i = match rule_key.as_str() {
            "type" => 0,
            "color" => 1,
            "name" => 2,
            _ => unreachable!(),
        };

        items
            .into_iter()
            .filter(|item| item[i] == rule_value)
            .count() as i32
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_1773() {
        assert_eq!(
            Solution::count_matches(
                vec![
                    vec!["phone".to_string(), "blue".to_string(), "pixel".to_string()],
                    vec![
                        "computer".to_string(),
                        "silver".to_string(),
                        "lenovo".to_string()
                    ],
                    vec![
                        "phone".to_string(),
                        "gold".to_string(),
                        "iphone".to_string()
                    ]
                ],
                "color".to_string(),
                "silver".to_string()
            ),
            1
        );
        assert_eq!(
            Solution::count_matches(
                vec![
                    vec!["phone".to_string(), "blue".to_string(), "pixel".to_string()],
                    vec![
                        "computer".to_string(),
                        "silver".to_string(),
                        "phone".to_string()
                    ],
                    vec![
                        "phone".to_string(),
                        "gold".to_string(),
                        "iphone".to_string()
                    ]
                ],
                "type".to_string(),
                "phone".to_string()
            ),
            2
        );
    }
}
