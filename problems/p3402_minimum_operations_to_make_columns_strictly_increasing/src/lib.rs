pub struct Solution {}

impl Solution {
    pub fn minimum_operations(grid: Vec<Vec<i32>>) -> i32 {
        let mut grid = grid;
        let mut operations = 0;

        for j in 0..grid[0].len() {
            for i in 1..grid.len() {
                if grid[i][j] <= grid[i - 1][j] {
                    operations += grid[i - 1][j] - grid[i][j] + 1;
                    grid[i][j] = grid[i - 1][j] + 1;
                }
            }
        }

        operations
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_3402() {
        assert_eq!(
            Solution::minimum_operations(vec![vec![3, 2], vec![1, 3], vec![3, 4], vec![0, 1]]),
            15
        );
        assert_eq!(
            Solution::minimum_operations(vec![vec![3, 2, 1], vec![2, 1, 0], vec![1, 2, 3]]),
            12
        );
    }
}
