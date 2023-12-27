pub struct Solution {}

impl Solution {
    pub fn min_path_cost(grid: Vec<Vec<i32>>, move_cost: Vec<Vec<i32>>) -> i32 {
        let mut dp = grid[0].clone();

        for i in 1..grid.len() {
            let mut new_dp = vec![std::i32::MAX; grid[0].len()];
            for j in 0..grid[0].len() {
                for k in 0..grid[0].len() {
                    new_dp[k] =
                        new_dp[k].min(dp[j] + move_cost[grid[i - 1][j] as usize][k] + grid[i][k]);
                }
            }
            dp = new_dp;
        }

        *dp.iter().min().unwrap()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_2304() {
        assert_eq!(
            Solution::min_path_cost(
                vec![vec![5, 3], vec![4, 0], vec![2, 1]],
                vec![
                    vec![9, 8],
                    vec![1, 5],
                    vec![10, 12],
                    vec![18, 6],
                    vec![2, 4],
                    vec![14, 3]
                ]
            ),
            17
        );
        assert_eq!(
            Solution::min_path_cost(
                vec![vec![5, 1, 2], vec![4, 0, 3]],
                vec![
                    vec![12, 10, 15],
                    vec![20, 23, 8],
                    vec![21, 7, 1],
                    vec![8, 1, 13],
                    vec![9, 10, 25],
                    vec![5, 3, 2]
                ]
            ),
            6
        );
    }
}
