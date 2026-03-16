pub struct Solution {}

impl Solution {
    pub fn min_cost(m: i32, n: i32, wait_cost: Vec<Vec<i32>>) -> i64 {
        let (m, n) = (m as usize, n as usize);
        let mut dp = vec![vec![std::i64::MAX; n]; m];
        let mut wait_cost = wait_cost;
        wait_cost[0][0] = 0;
        dp[0][0] = 1;

        for i in 0..m {
            for j in 0..n {
                let entering_cost = (i + 1) as i64 * (j + 1) as i64;
                if 0 < i {
                    dp[i][j] =
                        dp[i][j].min(dp[i - 1][j] + wait_cost[i - 1][j] as i64 + entering_cost);
                }
                if 0 < j {
                    dp[i][j] =
                        dp[i][j].min(dp[i][j - 1] + wait_cost[i][j - 1] as i64 + entering_cost);
                }
            }
        }

        dp[m - 1][n - 1]
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_3603() {
        assert_eq!(Solution::min_cost(1, 2, vec![vec![1, 2]]), 3);
        assert_eq!(Solution::min_cost(2, 2, vec![vec![3, 5], vec![2, 4]]), 9);
        assert_eq!(
            Solution::min_cost(2, 3, vec![vec![6, 1, 4], vec![3, 2, 5]]),
            16
        );
    }
}
