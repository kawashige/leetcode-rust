pub struct Solution {}

impl Solution {
    pub fn recurse(s: &str, p: &str, i: usize, j: usize) -> bool {
        if j == p.len() || (j == p.len() - 1 && p.as_bytes()[j] == b'*') {
            return true;
        }
        if i == s.len() {
            return false;
        }
        if p.as_bytes()[j] != b'*' {
            if s.as_bytes()[i] == p.as_bytes()[j] {
                Self::recurse(s, p, i + 1, j + 1)
            } else {
                false
            }
        } else {
            Self::recurse(s, p, i + 1, j) || Self::recurse(s, p, i, j + 1)
        }
    }
    pub fn has_match(s: String, p: String) -> bool {
        for i in 0..s.len() {
            if Self::recurse(&s, &p, i, 0) {
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
    fn test_3407() {
        assert!(Solution::has_match(
            "leetcode".to_string(),
            "ee*e".to_string()
        ));
        assert!(!Solution::has_match("car".to_string(), "c*v".to_string()));
        assert!(Solution::has_match("luck".to_string(), "u*".to_string()));
    }
}
