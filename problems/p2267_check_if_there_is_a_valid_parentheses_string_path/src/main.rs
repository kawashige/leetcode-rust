pub struct Solution {}

impl Solution {
    pub fn has_valid_path(grid: Vec<Vec<char>>) -> bool {
        if (grid.len() + grid[0].len() - 1) % 2 == 1 {
            return false;
        }
        let mut dp = vec![
            vec![vec![false; (grid.len() + grid[0].len() - 1) / 2]; grid[0].len()];
            grid.len()
        ];

        if grid[0][0] == ')' {
            return false;
        }
        dp[0][0][1] = true;

        for i in 0..grid.len() {
            for j in 0..grid[0].len() {
                if (i, j) == (0, 0) {
                    continue;
                }
                let p = if grid[i][j] == '(' { 1 } else { -1 };
                for k in 0..dp[i][j]
                    .len()
                    .min(i + 1)
                    .min(grid.len() + grid[0].len() - (i + j))
                {
                    if 0 < i
                        && dp[i - 1][j][k]
                        && (0..dp[i][j].len() as i32).contains(&(k as i32 + p))
                    {
                        dp[i][j][(k as i32 + p) as usize] = true;
                    }
                    if 0 < j
                        && dp[i][j - 1][k]
                        && (0..dp[i][j].len() as i32).contains(&(k as i32 + p))
                    {
                        dp[i][j][(k as i32 + p) as usize] = true;
                    }
                }
            }
        }

        dp[grid.len() - 1][grid[0].len() - 1][0]
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_2267() {
        assert!(Solution::has_valid_path(vec![
            vec!['(', '(', '('],
            vec![')', '(', ')'],
            vec!['(', '(', ')'],
            vec!['(', '(', ')']
        ]));
        assert!(!Solution::has_valid_path(vec![
            vec![')', ')'],
            vec!['(', '(']
        ]));
    }
}

fn main() {}
