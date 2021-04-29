pub struct Solution {}

impl Solution {
    pub fn max_increase_keeping_skyline(grid: Vec<Vec<i32>>) -> i32 {
        let mut row_max = vec![0; grid.len()];
        let mut column_max = vec![0; grid[0].len()];

        for i in 0..grid.len() {
            for j in 0..grid[0].len() {
                row_max[i] = std::cmp::max(row_max[i], grid[i][j]);
                column_max[j] = std::cmp::max(column_max[j], grid[i][j]);
            }
        }

        let mut sum = 0;
        for i in 0..grid.len() {
            for j in 0..grid[0].len() {
                sum += std::cmp::min(row_max[i], column_max[j]) - grid[i][j];
            }
        }

        sum
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_0807() {
        assert_eq!(
            Solution::max_increase_keeping_skyline(vec![
                vec![3, 0, 8, 4],
                vec![2, 4, 5, 7],
                vec![9, 2, 6, 3],
                vec![0, 3, 1, 0]
            ]),
            35
        )
    }
}
