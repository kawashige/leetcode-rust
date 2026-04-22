pub struct Solution {}

impl Solution {
    pub fn unique_paths(grid: Vec<Vec<i32>>) -> i32 {
        const M: usize = 1_000_000_007;
        let mut dp = vec![vec![vec![0; 2]; grid[0].len()]; grid.len()];
        dp[0][1][0] = 1;
        dp[1][0][1] = 1;

        for i in 0..grid.len() {
            for j in 0..grid[0].len() {
                if 0 < j {
                    if grid[i][j - 1] == 1 {
                        dp[i][j][0] += dp[i][j - 1][1];
                    } else {
                        dp[i][j][0] += dp[i][j - 1][0] + dp[i][j - 1][1];
                    }
                    dp[i][j][0] %= M;
                }
                if 0 < i {
                    if grid[i - 1][j] == 1 {
                        dp[i][j][1] += dp[i - 1][j][0];
                    } else {
                        dp[i][j][1] += dp[i - 1][j][0] + dp[i - 1][j][1];
                    }
                    dp[i][j][1] %= M;
                }
            }
        }

        (dp[grid.len() - 1][grid[0].len() - 1].iter().sum::<usize>() % M) as i32
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_3665() {
        assert_eq!(
            Solution::unique_paths(vec![vec![0, 1, 0], vec![0, 0, 1], vec![1, 0, 0]]),
            5
        );
        assert_eq!(Solution::unique_paths(vec![vec![1, 0], vec![0, 0]]), 2);
        assert_eq!(
            Solution::unique_paths(vec![vec![0, 1, 1], vec![1, 1, 0]]),
            1
        );
    }
}
