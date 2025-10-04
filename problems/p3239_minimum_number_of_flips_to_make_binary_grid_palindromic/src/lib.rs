pub struct Solution {}

impl Solution {
    pub fn min_flips(grid: Vec<Vec<i32>>) -> i32 {
        let mut row = 0;
        for i in 0..grid.len() {
            for j in 0..grid[0].len() / 2 {
                let mut count = [0; 2];
                count[grid[i][j] as usize] += 1;
                count[grid[i][grid[0].len() - 1 - j] as usize] += 1;
                row += count[0].min(count[1]);
            }
        }

        let mut column = 0;
        for j in 0..grid[0].len() {
            for i in 0..grid.len() / 2 {
                let mut count = [0; 2];
                count[grid[i][j] as usize] += 1;
                count[grid[grid.len() - 1 - i][j] as usize] += 1;
                column += count[0].min(count[1]);
            }
        }

        row.min(column)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_3239() {
        assert_eq!(
            Solution::min_flips(vec![vec![1, 0, 0], vec![0, 0, 0], vec![0, 0, 1]]),
            2
        );
        assert_eq!(
            Solution::min_flips(vec![vec![0, 1], vec![0, 1], vec![0, 0]]),
            1
        );
    }
}
