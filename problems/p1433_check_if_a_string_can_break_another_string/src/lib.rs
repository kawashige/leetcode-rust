use std::collections::HashSet;

pub struct Solution {}

impl Solution {
    pub fn count_chars(s: &str) -> [usize; 26] {
        s.as_bytes().iter().fold([0; 26], |mut count, b| {
            count[(b - b'a') as usize] += 1;
            count
        })
    }

    pub fn check_if_can_break(s1: String, s2: String) -> bool {
        let mut s1_chars = Self::count_chars(s1.as_str());
        let mut s2_chars = Self::count_chars(s2.as_str());

        (0..26)
            .rev()
            .map(|i| {
                if i < 25 {
                    s1_chars[i] += s1_chars[i + 1];
                    s2_chars[i] += s2_chars[i + 1];
                }

                (s1_chars[i] as i32 - s2_chars[i] as i32).signum()
            })
            .filter(|c| c != &0)
            .collect::<HashSet<_>>()
            .len()
            < 2
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_1433() {
        assert!(!Solution::check_if_can_break(
            "qvgjjsp".to_string(),
            "qmsbphx".to_string()
        ));
        assert!(Solution::check_if_can_break(
            "abc".to_string(),
            "xya".to_string()
        ));
        assert!(!Solution::check_if_can_break(
            "abe".to_string(),
            "acd".to_string()
        ));
        assert!(Solution::check_if_can_break(
            "leetcode".to_string(),
            "interview".to_string()
        ));
    }
}
