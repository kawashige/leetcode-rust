pub struct Solution {}

impl Solution {
    pub fn grid_game(grid: Vec<Vec<i32>>) -> i64 {
        let mut sum = vec![vec![0; grid[0].len()]; 2];
        sum[0][0] = grid[0][0] as i64;
        sum[1][0] = grid[1][0] as i64;
        for i in 1..grid[0].len() {
            sum[0][i] = grid[0][i] as i64 + sum[0][i - 1];
            sum[1][i] = grid[1][i] as i64 + sum[1][i - 1];
        }

        let mut min = std::i64::MAX;

        for i in 0..grid[0].len() {
            let up = if i < grid[0].len() - 1 {
                sum[0][sum[0].len() - 1] - sum[0][i]
            } else {
                0
            };
            let down = if 0 < i { sum[1][i - 1] } else { 0 };

            min = min.min(up.max(down));
        }

        min
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_2017() {
        assert_eq!(Solution::grid_game(vec![vec![2, 5, 4], vec![1, 5, 1]]), 4);
        assert_eq!(Solution::grid_game(vec![vec![3, 3, 1], vec![8, 5, 2]]), 4);
        assert_eq!(
            Solution::grid_game(vec![vec![1, 3, 1, 15], vec![1, 3, 3, 1]]),
            7
        );
    }
}
