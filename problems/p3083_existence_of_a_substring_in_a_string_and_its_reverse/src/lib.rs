pub struct Solution {}

impl Solution {
    pub fn is_substring_present(s: String) -> bool {
        let reverse = s.chars().rev().collect::<String>();
        for i in 0..s.len() - 1 {
            if reverse.contains(&s[i..i + 2]) {
                return true;
            }
        }
        false
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_3083() {
        assert!(Solution::is_substring_present("leetcode".to_string()));
        assert!(Solution::is_substring_present("abcba".to_string()));
        assert!(!Solution::is_substring_present("abcd".to_string()));
    }
}
