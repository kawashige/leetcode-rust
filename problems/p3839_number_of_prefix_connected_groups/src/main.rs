use std::collections::HashMap;

pub struct Solution {}

impl Solution {
    pub fn prefix_connected(words: Vec<String>, k: i32) -> i32 {
        let k = k as usize;
        let mut map = HashMap::new();
        let mut result = 0;

        for w in words {
            if w.len() < k {
                continue;
            }
            *map.entry(w[..k].to_string()).or_insert(0) += 1;
            if map.get(&w[..k]) == Some(&2) {
                result += 1;
            }
        }

        result
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_3839() {
        assert_eq!(
            Solution::prefix_connected(
                vec![
                    "apple".to_string(),
                    "apply".to_string(),
                    "banana".to_string(),
                    "bandit".to_string()
                ],
                2
            ),
            2
        );
        assert_eq!(
            Solution::prefix_connected(
                vec!["car".to_string(), "cat".to_string(), "cartoon".to_string()],
                3
            ),
            1
        );
        assert_eq!(
            Solution::prefix_connected(
                vec![
                    "bat".to_string(),
                    "dog".to_string(),
                    "dog".to_string(),
                    "doggy".to_string(),
                    "bat".to_string()
                ],
                3
            ),
            2
        );
    }
}

fn main() {}
