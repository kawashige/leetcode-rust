pub struct Solution {}

impl Solution {
    pub fn is_match(s: String, p: String) -> bool {
        let mut p2: Vec<String> = Vec::new();
        for i in 0..p.len() {
            if p.as_bytes()[i] == b'*' {
                p2.last_mut().unwrap().push(p.as_bytes()[i] as char);
            } else {
                p2.push(format!("{}", p.as_bytes()[i] as char));
            }
        }
        let mut dp = vec![vec![false; p2.len() + 1]; s.len() + 1];
        dp[0][0] = true;

        for i in 0..s.len() {
            for j in 0..p2.len() {
                if dp[i][j] {
                    if p2[j].len() == 1 {
                        if p2[j].as_bytes()[0] == b'.' || s.as_bytes()[i] == p2[j].as_bytes()[0] {
                            dp[i + 1][j + 1] = true;
                        }
                    } else {
                        dp[i][j + 1] = true;
                        let mut k = i;
                        while k < s.len()
                            && (p2[j].as_bytes()[0] == b'.'
                                || s.as_bytes()[k] == p2[j].as_bytes()[0])
                        {
                            dp[k + 1][j + 1] = true;
                            k += 1;
                        }
                    }
                }
                if p2[j].len() == 2 && dp[i + 1][j] {
                    dp[i + 1][j + 1] = true;
                }
            }
        }

        dp[dp.len() - 1][dp[0].len() - 1]
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_0010() {
        assert!(Solution::is_match(
            "aabcbcbcaccbcaabc".to_string(),
            ".*a*aa*.*b*.c*.*a*".to_string()
        ));
        assert!(Solution::is_match("a".to_string(), "ab*".to_string()));
        assert!(!Solution::is_match("aa".to_string(), "a".to_string()));
        assert!(Solution::is_match("aa".to_string(), "a*".to_string()));
        assert!(Solution::is_match("ab".to_string(), ".*".to_string()));
    }
}
