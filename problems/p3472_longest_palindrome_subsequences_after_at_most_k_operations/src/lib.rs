pub struct Solution {}

impl Solution {
    pub fn longest_palindromic_subsequence(s: String, k: i32) -> i32 {
        let mut result = 1;
        let k = k as usize;
        let mut dp = vec![vec![vec![0; k + 1]; s.len()]; s.len()];
        for i in 0..s.len() {
            dp[i][i][0] = 1;
            if i != s.len() - 1 {
                let d =
                    ((s.as_bytes()[i] - b'a') as i32 - (s.as_bytes()[i + 1] - b'a') as i32).abs();
                let d = d.min(26 - d) as usize;
                if d <= k {
                    dp[i][i + 1][d] = 2;
                    result = 2;
                }
                for j in 0..=k {
                    dp[i][i + 1][j] = dp[i][i + 1][j].max(1);
                }
            }
        }

        for l in 2..s.len() {
            for i in 0..s.len() - l {
                let d =
                    ((s.as_bytes()[i] - b'a') as i32 - (s.as_bytes()[i + l] - b'a') as i32).abs();
                let d = d.min(26 - d) as usize;
                for j in 0..=k {
                    dp[i][i + l][j] = dp[i][i + l - 1][j].max(dp[i + 1][i + l][j]);
                    if d <= j {
                        dp[i][i + l][j] = dp[i][i + l][j].max(dp[i + 1][i + l - 1][j - d] + 2);
                    }
                    result = result.max(dp[i][i + l][j]);
                }
            }
        }

        result
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_3472() {
        assert_eq!(
            Solution::longest_palindromic_subsequence("wehzr".to_string(), 3),
            3
        );
        assert_eq!(
            Solution::longest_palindromic_subsequence("abced".to_string(), 2),
            3
        );
        assert_eq!(
            Solution::longest_palindromic_subsequence("aaazzz".to_string(), 4),
            6
        );
    }
}
