pub struct Solution {}

impl Solution {
    pub fn num_magic_squares_inside(grid: Vec<Vec<i32>>) -> i32 {
        if grid.len() < 3 || grid[0].len() < 3 {
            return 0;
        }

        let mut count = 0;
        for i in 1..(grid.len() - 1) {
            for j in 1..(grid[0].len() - 1) {
                if grid[i][j] != 5 {
                    continue;
                }

                if [
                    [8, 1, 6, 7, 2, 9, 4, 3],
                    [8, 3, 4, 9, 2, 7, 6, 1],
                    [6, 7, 2, 9, 4, 3, 8, 1],
                    [6, 1, 8, 3, 4, 9, 2, 7],
                    [2, 9, 4, 3, 8, 1, 6, 7],
                    [2, 7, 6, 1, 8, 3, 4, 9],
                    [4, 3, 8, 1, 6, 7, 2, 9],
                    [4, 9, 2, 7, 6, 1, 8, 3],
                ]
                .contains(&[
                    grid[i - 1][j - 1],
                    grid[i - 1][j],
                    grid[i - 1][j + 1],
                    grid[i][j + 1],
                    grid[i + 1][j + 1],
                    grid[i + 1][j],
                    grid[i + 1][j - 1],
                    grid[i][j - 1],
                ]) {
                    count += 1;
                }
            }
        }
        count
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_00840() {
        assert_eq!(
            Solution::num_magic_squares_inside(vec![
                vec![4, 3, 8, 4],
                vec![9, 5, 1, 9],
                vec![2, 7, 6, 2]
            ]),
            1
        );
        assert_eq!(Solution::num_magic_squares_inside(vec![vec![8]]), 0);
        assert_eq!(
            Solution::num_magic_squares_inside(vec![vec![4, 4], vec![3, 3]]),
            0
        );
        assert_eq!(
            Solution::num_magic_squares_inside(vec![vec![4, 7, 8], vec![9, 5, 1], vec![2, 3, 6]]),
            0
        );
    }
}
