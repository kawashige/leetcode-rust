use std::collections::HashSet;

pub struct Solution {}

impl Solution {
    pub fn dest_city(paths: Vec<Vec<String>>) -> String {
        let from_cities = paths.iter().map(|path| &path[0]).collect::<HashSet<_>>();
        paths
            .iter()
            .find(|path| !from_cities.contains(&path[1]))
            .unwrap()[1]
            .clone()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_1436() {
        assert_eq!(
            Solution::dest_city(vec![
                vec!["London".to_string(), "New York".to_string()],
                vec!["New York".to_string(), "Lima".to_string()],
                vec!["Lima".to_string(), "Sao Paulo".to_string()]
            ]),
            "Sao Paulo".to_string()
        );
        assert_eq!(
            Solution::dest_city(vec![
                vec!["B".to_string(), "C".to_string()],
                vec!["D".to_string(), "B".to_string()],
                vec!["C".to_string(), "A".to_string()]
            ]),
            "A".to_string()
        );
        assert_eq!(
            Solution::dest_city(vec![vec!["A".to_string(), "Z".to_string()]]),
            "Z".to_string()
        );
    }
}
