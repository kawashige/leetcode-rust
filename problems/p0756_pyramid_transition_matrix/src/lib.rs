use std::collections::HashMap;

pub struct Solution {}

impl Solution {
    pub fn dfs(
        prev: &[char],
        next: &mut Vec<char>,
        map: &HashMap<(char, char), Vec<char>>,
    ) -> bool {
        if prev.len() == next.len() + 1 {
            if next.len() == 1 {
                return true;
            } else {
                return Self::dfs(&next, &mut vec![], map);
            }
        }

        let i = next.len();
        if let Some(chars) = map.get(&(prev[i], prev[i + 1])) {
            for c in chars {
                next.push(*c);
                if Self::dfs(prev, next, map) {
                    return true;
                }
                next.pop();
            }
        }

        false
    }

    pub fn pyramid_transition(bottom: String, allowed: Vec<String>) -> bool {
        let mut map = HashMap::new();
        for allow in allowed {
            let chars = allow.chars().collect::<Vec<char>>();
            (*map.entry((chars[0], chars[1])).or_insert(vec![])).push(chars[2]);
        }

        let prev = bottom.chars().collect::<Vec<char>>();
        Self::dfs(&prev, &mut vec![], &map)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_0756() {
        assert!(Solution::pyramid_transition(
            "BCD".to_string(),
            vec![
                "BCG".to_string(),
                "CDE".to_string(),
                "GEA".to_string(),
                "FFF".to_string()
            ]
        ));

        assert!(!Solution::pyramid_transition(
            "AABA".to_string(),
            vec![
                "AAA".to_string(),
                "AAB".to_string(),
                "ABA".to_string(),
                "ABB".to_string(),
                "BAC".to_string()
            ]
        ));
    }
}
