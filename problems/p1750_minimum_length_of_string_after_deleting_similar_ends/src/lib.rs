use std::collections::VecDeque;

pub struct Solution {}

impl Solution {
    pub fn minimum_length(s: String) -> i32 {
        let mut s = s.chars().collect::<VecDeque<_>>();

        while 1 < s.len() && s[0] == s[s.len() - 1] {
            let target = s[0];
            while !s.is_empty() && s[0] == target {
                s.pop_front();
            }
            while !s.is_empty() && s[s.len() - 1] == target {
                s.pop_back();
            }
        }

        s.len() as i32
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_1750() {
        assert_eq!(Solution::minimum_length("a".to_string()), 1);
        assert_eq!(Solution::minimum_length("aa".to_string()), 0);
        assert_eq!(Solution::minimum_length("ca".to_string()), 2);
        assert_eq!(Solution::minimum_length("cabaabac".to_string()), 0);
        assert_eq!(Solution::minimum_length("aabccabba".to_string()), 3);
    }
}
