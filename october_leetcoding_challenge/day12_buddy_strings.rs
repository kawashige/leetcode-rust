pub struct Solution {}

use std::collections::HashSet;
impl Solution {
    pub fn buddy_strings(a: String, b: String) -> bool {
        if a.len() != b.len() || a.len() < 2 {
            return false;
        }

        if a == b {
            a.len() != a.chars().collect::<HashSet<char>>().len()
        } else {
            let diff = a
                .chars()
                .zip(b.chars())
                .filter(|(c1, c2)| c1 != c2)
                .collect::<Vec<(char, char)>>();
            diff.len() == 2 && diff[0].0 == diff[1].1 && diff[0].1 == diff[1].0
        }
    }
}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn test_day12() {
        assert!(Solution::buddy_strings("ab".to_string(), "ba".to_string()));
        assert!(Solution::buddy_strings(
            "aaaaaaabc".to_string(),
            "aaaaaaacb".to_string()
        ));
        assert!(!Solution::buddy_strings("".to_string(), "aa".to_string()));
        assert!(!Solution::buddy_strings("ab".to_string(), "ab".to_string()));
        assert!(Solution::buddy_strings("aa".to_string(), "aa".to_string()));
        assert!(!Solution::buddy_strings(
            "aazaaaabc".to_string(),
            "aaaazaacb".to_string()
        ));
    }
}
