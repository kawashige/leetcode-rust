pub struct Solution {}

impl Solution {
    pub fn min_falling_path_sum(grid: Vec<Vec<i32>>) -> i32 {
        let mut dp = grid[0].clone();

        for i in 1..grid.len() {
            let mut new_dp = vec![std::i32::MAX; grid[i].len()];

            for j in 0..grid[i].len() {
                for k in 0..grid[i].len() {
                    if j == k {
                        continue;
                    }
                    new_dp[j] = new_dp[j].min(dp[k] + grid[i][j]);
                }
            }

            dp = new_dp;
        }

        dp.into_iter().min().unwrap()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_1289() {
        assert_eq!(
            Solution::min_falling_path_sum(vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]]),
            13
        );
        assert_eq!(Solution::min_falling_path_sum(vec![vec![7]]), 7);
    }
}
