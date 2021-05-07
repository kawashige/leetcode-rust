pub struct Solution {}

impl Solution {
    pub fn is_interleave(s1: String, s2: String, s3: String) -> bool {
        if s1.len() + s2.len() != s3.len() {
            return false;
        }
        let mut dp = vec![false; s2.len() + 1];

        for i in 0..=s1.len() {
            for j in 0..=s2.len() {
                dp[j] = if i == 0 && j == 0 {
                    true
                } else if i == 0 {
                    dp[j - 1] && s2.as_bytes()[j - 1] == s3.as_bytes()[i + j - 1]
                } else if j == 0 {
                    dp[j] && s1.as_bytes()[i - 1] == s3.as_bytes()[i + j - 1]
                } else {
                    dp[j] && s1.as_bytes()[i - 1] == s3.as_bytes()[i + j - 1]
                        || dp[j - 1] && s2.as_bytes()[j - 1] == s3.as_bytes()[i + j - 1]
                };
            }
        }
        dp[s2.len()]
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_0097() {
        assert!(Solution::is_interleave(
            "aabaac".to_string(),
            "aadaaeaaf".to_string(),
            "aadaaeaabaafaac".to_string()
        ));
        assert!(!Solution::is_interleave(
            "db".to_string(),
            "b".to_string(),
            "cbb".to_string()
        ));
        assert!(Solution::is_interleave(
            "aabcc".to_string(),
            "dbbca".to_string(),
            "aadbbcbcac".to_string()
        ));
        assert!(!Solution::is_interleave(
            "aabcc".to_string(),
            "dbbca".to_string(),
            "aadbbbaccc".to_string()
        ));
        assert!(Solution::is_interleave(
            "".to_string(),
            "".to_string(),
            "".to_string()
        ));
    }
}
