pub struct Solution {}

impl Solution {
    pub fn num_ways(steps: i32, arr_len: i32) -> i32 {
        const M: i32 = 1_000_000_007;

        let mut dp = vec![vec![0; steps.min(arr_len) as usize]; steps as usize + 1];
        dp[0][0] = 1;

        for i in 0..steps as usize {
            for j in 0..dp[i].len() {
                dp[i + 1][j] += dp[i][j];
                dp[i + 1][j] %= M;
                if 0 < j {
                    dp[i + 1][j - 1] += dp[i][j];
                    dp[i + 1][j - 1] %= M;
                }
                if j < dp[i].len() - 1 {
                    dp[i + 1][j + 1] += dp[i][j];
                    dp[i + 1][j + 1] %= M;
                }
            }
        }

        dp[steps as usize][0]
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_1269() {
        assert_eq!(Solution::num_ways(3, 2), 4);
        assert_eq!(Solution::num_ways(2, 4), 2);
        assert_eq!(Solution::num_ways(4, 2), 8);
    }
}
