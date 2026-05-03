pub struct Solution {}

impl Solution {
    pub fn max_path_score(grid: Vec<Vec<i32>>, k: i32) -> i32 {
        let k = k as usize;
        let mut dp = vec![vec![vec![-1; k as usize + 1]; grid[0].len()]; grid.len()];
        dp[0][0][grid[0][0].min(1) as usize] = grid[0][0];

        for i in 0..grid.len() {
            for j in 0..grid[0].len() {
                for c in 0..=k {
                    let new_c = c + grid[i][j].min(1) as usize;
                    if k < new_c {
                        continue;
                    }
                    if 0 < i && dp[i - 1][j][c] != -1 {
                        dp[i][j][new_c] = dp[i][j][new_c].max(dp[i - 1][j][c] + grid[i][j]);
                    }
                    if 0 < j && dp[i][j - 1][c] != -1 {
                        dp[i][j][new_c] = dp[i][j][new_c].max(dp[i][j - 1][c] + grid[i][j]);
                    }
                }
            }
        }

        *dp[dp.len() - 1][dp[0].len() - 1].iter().max().unwrap()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_3742() {
        assert_eq!(Solution::max_path_score(vec![vec![0, 1], vec![2, 0]], 1), 2);
        assert_eq!(
            Solution::max_path_score(vec![vec![0, 1], vec![1, 2]], 1),
            -1
        );
    }
}

fn main() {}
