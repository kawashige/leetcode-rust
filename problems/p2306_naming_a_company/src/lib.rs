use std::collections::HashMap;

pub struct Solution {}

impl Solution {
    pub fn distinct_names(ideas: Vec<String>) -> i64 {
        let map = ideas.iter().fold(HashMap::new(), |mut map, idea| {
            *map.entry(idea[1..].to_string()).or_insert(0_usize) |=
                1 << (idea.as_bytes()[0] - b'a') as usize;
            map
        });

        let mut count = [[0; 26]; 26];

        for idea in &ideas {
            for i in 0..26 {
                if map[&idea[1..]] & 1 << i == 0 {
                    count[(idea.as_bytes()[0] - b'a') as usize][i] += 1;
                }
            }
        }

        let mut result = 0;

        for idea in ideas {
            for i in 0..26 {
                if map[&idea[1..]] & 1 << i == 0 {
                    result += count[i][(idea.as_bytes()[0] - b'a') as usize];
                }
            }
        }

        result
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_2306() {
        assert_eq!(
            Solution::distinct_names(vec![
                "coffee".to_string(),
                "donuts".to_string(),
                "time".to_string(),
                "toffee".to_string()
            ]),
            6
        );
        assert_eq!(
            Solution::distinct_names(vec!["lack".to_string(), "back".to_string()]),
            0
        );
    }
}
