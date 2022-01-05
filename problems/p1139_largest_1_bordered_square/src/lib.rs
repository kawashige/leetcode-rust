pub struct Solution {}

impl Solution {
    pub fn largest1_bordered_square(grid: Vec<Vec<i32>>) -> i32 {
        let mut left = vec![vec![0; grid[0].len()]; grid.len()];
        let mut up = vec![vec![0; grid[0].len()]; grid.len()];

        for i in 0..grid.len() {
            for j in 0..grid[0].len() {
                if grid[i][j] == 1 {
                    left[i][j] = 1;
                    up[i][j] = 1;
                    if j > 0 {
                        left[i][j] += left[i][j - 1];
                    }
                    if i > 0 {
                        up[i][j] += up[i - 1][j];
                    }
                }
            }
        }

        let mut result = 0;

        for i in 0..grid.len() {
            for j in 0..grid[0].len() {
                for k in 0..(grid.len() - i).min(grid[0].len() - j) {
                    if k < left[i][j + k]
                        && k < up[i + k][j]
                        && k < left[i + k][j + k]
                        && k < up[i + k][j + k]
                    {
                        result = result.max(k + 1)
                    }
                }
            }
        }

        (result * result) as i32
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_1139() {
        assert_eq!(
            Solution::largest1_bordered_square(vec![
                vec![1, 1, 1],
                vec![1, 1, 0],
                vec![1, 1, 1],
                vec![0, 1, 1],
                vec![1, 1, 1]
            ]),
            4
        );
        assert_eq!(
            Solution::largest1_bordered_square(vec![vec![1, 1, 1], vec![1, 0, 1], vec![1, 1, 1]]),
            9
        );
        assert_eq!(
            Solution::largest1_bordered_square(vec![vec![1, 1, 0, 0]]),
            1
        );
    }
}
