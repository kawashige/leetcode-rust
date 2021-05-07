pub struct Solution {}

impl Solution {
    pub fn is_interleave(s1: String, s2: String, s3: String) -> bool {
        if s1.len() + s2.len() != s3.len() {
            return false;
        }

        if s1.is_empty() {
            return s2 == s3;
        } else if s2.is_empty() {
            return s1 == s3;
        }

        let mut dp = vec![vec![vec![false; s2.len() + 2]; s1.len() + 2]; s3.len() + 2];
        dp[0][0][0] = true;

        for i in 1..=s3.len() {
            for j in 0..=i {
                if j > s1.len() || i - j > s2.len() {
                    continue;
                }
                dp[i][j][i - j] = (j > 0
                    && s1.as_bytes()[j - 1] == s3.as_bytes()[i - 1]
                    && dp[i - 1][j - 1][i - j])
                    || (i - j > 0
                        && s2.as_bytes()[i - j - 1] == s3.as_bytes()[i - 1]
                        && dp[i - 1][j][i - j - 1]);
            }
        }

        dp[s3.len()][s1.len()][s2.len()]
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
