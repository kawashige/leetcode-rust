pub struct Solution {}
use std::cmp::max;
impl Solution {
    pub fn maximum_profit(prices: Vec<i32>, k: i32) -> i64 {
        let n = prices.len();
        let k = k as usize;
        let mut dp = vec![vec![vec![0i64; 3]; k + 1]; n];

        // initialize the state on day 0
        for j in 1..=k {
            dp[0][j][1] = -(prices[0] as i64);
            dp[0][j][2] = prices[0] as i64;
        }

        for i in 1..n {
            let val = prices[i] as i64;
            for j in 1..=k {
                dp[i][j][0] = max(
                    dp[i - 1][j][0],
                    max(dp[i - 1][j][1] + val, dp[i - 1][j][2] - val),
                );
                dp[i][j][1] = max(dp[i - 1][j][1], dp[i - 1][j - 1][0] - val);
                dp[i][j][2] = max(dp[i - 1][j][2], dp[i - 1][j - 1][0] + val);
            }
        }

        dp[n - 1][k][0]
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_3573() {
        assert_eq!(Solution::maximum_profit(vec![1, 7, 9, 8, 2], 2), 14);
        assert_eq!(
            Solution::maximum_profit(vec![12, 16, 19, 19, 8, 1, 19, 13, 9], 3),
            36
        );
    }
}
