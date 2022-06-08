use std::collections::HashMap;

pub struct Solution {}

impl Solution {
    pub fn get_folder_names(names: Vec<String>) -> Vec<String> {
        let mut map = HashMap::new();
        names
            .into_iter()
            .map(|name| -> String {
                if let Some(i) = map.get(&name) {
                    let mut target = *i;
                    while map.contains_key(&format!("{}({})", name, target)) {
                        target += 1;
                    }
                    let new_name = format!("{}({})", name, target);
                    map.insert(new_name.clone(), 1);
                    map.insert(name, target + 1);
                    new_name
                } else {
                    map.insert(name.clone(), 1);
                    name
                }
            })
            .collect()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_1487() {
        assert_eq!(
            Solution::get_folder_names(vec![
                "pes".to_string(),
                "fifa".to_string(),
                "gta".to_string(),
                "pes(2019)".to_string()
            ]),
            vec![
                "pes".to_string(),
                "fifa".to_string(),
                "gta".to_string(),
                "pes(2019)".to_string()
            ]
        );
        assert_eq!(
            Solution::get_folder_names(vec![
                "gta".to_string(),
                "gta(1)".to_string(),
                "gta".to_string(),
                "avalon".to_string()
            ]),
            vec![
                "gta".to_string(),
                "gta(1)".to_string(),
                "gta(2)".to_string(),
                "avalon".to_string()
            ]
        );
        assert_eq!(
            Solution::get_folder_names(vec![
                "onepiece".to_string(),
                "onepiece(1)".to_string(),
                "onepiece(2)".to_string(),
                "onepiece(3)".to_string(),
                "onepiece".to_string()
            ]),
            vec![
                "onepiece".to_string(),
                "onepiece(1)".to_string(),
                "onepiece(2)".to_string(),
                "onepiece(3)".to_string(),
                "onepiece(4)".to_string()
            ]
        );
    }
}
