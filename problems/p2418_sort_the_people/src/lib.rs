pub struct Solution {}

impl Solution {
    pub fn sort_people(names: Vec<String>, heights: Vec<i32>) -> Vec<String> {
        let mut people = heights
            .into_iter()
            .zip(names.into_iter())
            .collect::<Vec<_>>();
        people.sort_unstable_by(|a, b| b.0.cmp(&a.0));
        people.into_iter().map(|(_, name)| name).collect()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_2418() {
        assert_eq!(
            Solution::sort_people(
                vec!["Mary".to_string(), "John".to_string(), "Emma".to_string()],
                vec![180, 165, 170]
            ),
            vec!["Mary".to_string(), "Emma".to_string(), "John".to_string()]
        );
        assert_eq!(
            Solution::sort_people(
                vec!["Alice".to_string(), "Bob".to_string(), "Bob".to_string()],
                vec![155, 185, 150]
            ),
            vec!["Bob".to_string(), "Alice".to_string(), "Bob".to_string()]
        );
    }
}
