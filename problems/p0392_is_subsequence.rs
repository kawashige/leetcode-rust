pub struct Solution {}

impl Solution {
    pub fn is_subsequence(s: String, t: String) -> bool {
        if t.len() < s.len() {
            return false;
        }
        let mut i = 0;
        let mut j = 0;
        let ss = s.as_bytes();
        let tt = t.as_bytes();
        while i < s.len() && j < t.len() {
            if ss[i] == tt[j] {
                i += 1;
                j += 1;
            } else {
                j += 1;
            }
        }
        i == s.len()
    }
}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn test_0392() {
        assert!(Solution::is_subsequence(
            "abc".to_string(),
            "ahbgdc".to_string()
        ));
        assert!(!Solution::is_subsequence(
            "axc".to_string(),
            "ahbgdc".to_string()
        ));
        assert!(Solution::is_subsequence(
            "".to_string(),
            "ahbgdc".to_string()
        ));
        assert!(!Solution::is_subsequence("a".to_string(), "".to_string()));
    }
}
