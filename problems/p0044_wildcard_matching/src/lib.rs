pub struct Solution {}

impl Solution {
    pub fn is_match(s: String, p: String) -> bool {
        if p.is_empty() {
            return s.is_empty();
        }
        if s.is_empty() {
            return p.as_bytes().iter().all(|b| b == &b'*');
        }
        let mut dp = vec![vec![false; p.len() + 1]; s.len() + 1];
        dp[0][0] = true;
        for i in 0..p.len() {
            if p.as_bytes()[i] != b'*' {
                break;
            }
            dp[0][i + 1] = true;
        }

        for i in 0..s.len() {
            for j in 0..p.len() {
                if p.as_bytes()[j] == b'*' {
                    dp[i + 1][j + 1] = dp[i + 1][j];
                }
                if dp[i][j]
                    && (p.as_bytes()[j] == b'?'
                        || p.as_bytes()[j] == b'*'
                        || s.as_bytes()[i] == p.as_bytes()[j])
                {
                    dp[i + 1][j + 1] = true;
                }
                if p.as_bytes()[j] == b'*' && dp[i][j + 1] {
                    dp[i + 1][j + 1] = true;
                }
            }
        }

        dp[s.len()][p.len()]
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_0044() {
        assert!(Solution::is_match("adceb".to_string(), "*a*b".to_string()));
        assert!(!Solution::is_match("aa".to_string(), "a".to_string()));
        assert!(Solution::is_match("aa".to_string(), "*".to_string()));
        assert!(!Solution::is_match("cb".to_string(), "?a".to_string()));
    }
}
