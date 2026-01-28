pub struct Solution {}

impl Solution {
    pub fn min_cost(grid: Vec<Vec<i32>>, k: i32) -> i32 {
        let k = k as usize;
        let mut dp = vec![vec![vec![std::i32::MAX; k + 1]; grid[0].len()]; grid.len()];
        dp[0][0][k] = 0;

        for l in (0..=k).rev() {
            let mut acc = vec![std::i32::MAX; 100_001];
            if l < k {
                for i in 0..grid.len() {
                    for j in 0..grid[0].len() {
                        acc[grid[i][j] as usize] = acc[grid[i][j] as usize].min(dp[i][j][l + 1]);
                    }
                }
                for i in (0..acc.len() - 1).rev() {
                    acc[i] = acc[i].min(acc[i + 1]);
                }
            }
            for i in 0..grid.len() {
                for j in 0..grid[0].len() {
                    if 0 < i {
                        dp[i][j][l] = dp[i][j][l].min(dp[i - 1][j][l] + grid[i][j]);
                    }
                    if 0 < j {
                        dp[i][j][l] = dp[i][j][l].min(dp[i][j - 1][l] + grid[i][j]);
                    }
                    if l < k {
                        dp[i][j][l] = dp[i][j][l].min(acc[grid[i][j] as usize]);
                    }
                }
            }
        }

        *dp[dp.len() - 1][dp[0].len() - 1].iter().min().unwrap()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_3651() {
        assert_eq!(
            Solution::min_cost(vec![vec![1, 3, 3], vec![2, 5, 4], vec![4, 3, 5]], 2),
            7
        );
        assert_eq!(
            Solution::min_cost(vec![vec![1, 2], vec![2, 3], vec![3, 4]], 1),
            9
        );
    }
}
