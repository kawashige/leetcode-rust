pub struct Solution {}

impl Solution {
    pub fn sort_matrix(grid: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let mut grid = grid;

        for i in 0..grid.len() {
            let mut values = Vec::new();
            for j in 0..grid.len() - i {
                values.push(grid[i + j][j]);
            }
            values.sort_unstable();
            for j in 0..grid.len() - i {
                grid[i + j][j] = values.pop().unwrap();
            }
        }

        for i in 1..grid.len() {
            let mut values = Vec::new();
            for j in 0..grid.len() - i {
                values.push(grid[j][i + j]);
            }
            values.sort_unstable();
            for j in (0..grid.len() - i).rev() {
                grid[j][i + j] = values.pop().unwrap();
            }
        }

        grid
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_3446() {
        assert_eq!(
            Solution::sort_matrix(vec![vec![8, 2, 3], vec![9, 6, 7], vec![4, 5, 1]]),
            vec![vec![8, 2, 3], vec![9, 6, 7], vec![4, 5, 1]]
        );
        assert_eq!(
            Solution::sort_matrix(vec![vec![0, 1], vec![1, 2]]),
            vec![vec![2, 1], vec![1, 0]]
        );
    }
}
