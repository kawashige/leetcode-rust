pub struct Solution {}

impl Solution {
    pub fn min_flips(grid: Vec<Vec<i32>>) -> i32 {
        let mut result = 0;
        for i in 0..grid.len() / 2 {
            for j in 0..grid[0].len() / 2 {
                let mut count = [0; 2];
                count[grid[i][j] as usize] += 1;
                count[grid[i][grid[0].len() - 1 - j] as usize] += 1;
                count[grid[grid.len() - 1 - i][j] as usize] += 1;
                count[grid[grid.len() - 1 - i][grid[0].len() - 1 - j] as usize] += 1;
                result += count[0].min(count[1]);
                if (grid.len() % 2 == 1 && i == grid.len() / 2)
                    || (grid[0].len() % 2 == 1 && j == grid[0].len() / 2)
                {
                    count[grid[i][j] as usize] += 1;
                }
            }
        }

        let mut ones = 0;
        let mut diff = 0;
        if grid.len() % 2 == 1 {
            for j in 0..grid[0].len() / 2 {
                if grid[grid.len() / 2][j] != grid[grid.len() / 2][grid[0].len() - 1 - j] {
                    diff += 1;
                } else {
                    ones += grid[grid.len() / 2][j];
                }
            }
        }
        if grid[0].len() % 2 == 1 {
            for i in 0..grid.len() / 2 {
                if grid[i][grid[0].len() / 2] != grid[grid.len() - 1 - i][grid[0].len() / 2] {
                    diff += 1;
                } else {
                    ones += grid[i][grid[0].len() / 2];
                }
            }
        }

        result += diff
            + if (ones * 2) % 4 == 2 && diff == 0 {
                2
            } else {
                0
            };

        if grid.len() % 2 == 1 && grid[0].len() % 2 == 1 {
            result += grid[grid.len() / 2][grid[0].len() / 2];
        }

        result
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_3240() {
        assert_eq!(
            Solution::min_flips(vec![vec![1, 0, 0], vec![0, 1, 0], vec![0, 0, 1]]),
            3
        );
        assert_eq!(
            Solution::min_flips(vec![vec![0, 1], vec![0, 1], vec![0, 0]]),
            2
        );
        assert_eq!(Solution::min_flips(vec![vec![1], vec![1]]), 2);
    }
}
