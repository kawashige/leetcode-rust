pub struct Solution {}

impl Solution {
    pub fn ones_minus_zeros(grid: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let mut rows = vec![vec![0; grid.len()]; 2];
        let mut columns = vec![vec![0; grid[0].len()]; 2];

        for i in 0..grid.len() {
            for j in 0..grid[0].len() {
                rows[grid[i][j] as usize][i] += 1;
                columns[grid[i][j] as usize][j] += 1;
            }
        }

        let mut result = vec![vec![0; grid[0].len()]; grid.len()];
        for i in 0..grid.len() {
            for j in 0..grid[0].len() {
                result[i][j] = rows[1][i] + columns[1][j] - rows[0][i] - columns[0][j];
            }
        }
        result
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_2482() {
        assert_eq!(
            Solution::ones_minus_zeros(vec![vec![0, 1, 1], vec![1, 0, 1], vec![0, 0, 1]]),
            vec![vec![0, 0, 4], vec![0, 0, 4], vec![-2, -2, 2]]
        );
        assert_eq!(
            Solution::ones_minus_zeros(vec![vec![1, 1, 1], vec![1, 1, 1]]),
            vec![vec![5, 5, 5], vec![5, 5, 5]]
        );
    }
}
