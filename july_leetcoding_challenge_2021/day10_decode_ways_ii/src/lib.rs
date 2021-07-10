pub struct Solution {}

impl Solution {
    pub fn num_decodings(s: String) -> i32 {
        const M: usize = 1_000_000_007;
        let bytes = s.as_bytes();
        let mut dp = vec![0; s.len() + 1];
        dp[0] = 1;

        for i in 0..bytes.len() {
            match bytes[i] {
                b'0' if i == 0
                    || (bytes[i - 1] != b'1' && bytes[i - 1] != b'2' && bytes[i - 1] != b'*') =>
                {
                    return 0;
                }
                b'0' if bytes[i - 1] == b'*' => {
                    dp[i + 1] = dp[i - 1] * 2 % M;
                }
                b'0' => {
                    dp[i + 1] = dp[i - 1];
                }
                b'*' if i > 0 && bytes[i - 1] == b'*' => {
                    dp[i + 1] = (dp[i - 1] * 15 % M + dp[i] * 9 % M) % M;
                }
                b'*' if i > 0 && bytes[i - 1] == b'1' => {
                    dp[i + 1] = (dp[i - 1] * 9 % M + dp[i] * 9 % M) % M;
                }
                b'*' if i > 0 && bytes[i - 1] == b'2' => {
                    dp[i + 1] = (dp[i - 1] * 6 % M + dp[i] * 9 % M) % M;
                }
                b'*' => {
                    dp[i + 1] = dp[i] * 9 % M;
                }
                _ if i > 0 && bytes[i - 1] == b'*' && bytes[i] < b'7' => {
                    dp[i + 1] = (dp[i] + dp[i - 1] * 2 % M) % M;
                }
                _ if i > 0
                    && (bytes[i - 1] == b'*'
                        || bytes[i - 1] == b'1'
                        || (bytes[i - 1] == b'2' && bytes[i] < b'7')) =>
                {
                    dp[i + 1] = (dp[i] + dp[i - 1]) % M;
                }
                _ => {
                    dp[i + 1] = dp[i];
                }
            }
        }

        dp[s.len()] as i32
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_day10() {
        assert_eq!(Solution::num_decodings("*1*1*0".to_string()), 404);
        assert_eq!(Solution::num_decodings("*".to_string()), 9);
        assert_eq!(Solution::num_decodings("1*".to_string()), 18);
        assert_eq!(Solution::num_decodings("2*".to_string()), 15);
    }
}
