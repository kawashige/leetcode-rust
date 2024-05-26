pub struct Solution {}

impl Solution {
    pub fn check_record(n: i32) -> i32 {
        const M: usize = 1_000_000_007;

        let mut dp = vec![vec![0; 2]; 3];
        dp[0][0] = 1;

        for _ in 0..n {
            let mut new_dp = vec![vec![0; 2]; 3];
            for j in 0..dp.len() {
                for k in 0..dp[0].len() {
                    new_dp[0][k] += dp[j][k];
                    new_dp[0][k] %= M;
                    if j < dp.len() - 1 {
                        new_dp[j + 1][k] += dp[j][k];
                        new_dp[j + 1][k] %= M;
                    }
                    if k == 0 {
                        new_dp[0][k + 1] += dp[j][k];
                        new_dp[0][k + 1] %= M;
                    }
                }
            }
            dp = new_dp;
        }

        dp.into_iter()
            .map(|v| v.into_iter().fold(0, |acc, x| (acc + x) % M))
            .fold(0, |acc, x| (acc + x) % M) as i32
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_0552() {
        assert_eq!(Solution::check_record(2), 8);
        assert_eq!(Solution::check_record(1), 3);
        assert_eq!(Solution::check_record(10101), 183236316);
    }
}
