pub struct Solution {}

impl Solution {
    pub fn min_cost(grid: Vec<Vec<i32>>) -> i32 {
        let mut dp = vec![vec![[false; 1024]; grid[0].len()]; grid.len()];
        dp[0][0][grid[0][0] as usize] = true;

        for i in 0..grid.len() {
            for j in 0..grid[0].len() {
                if i == 0 && j == 0 {
                    continue;
                }
                for k in 0..dp[i][j].len() {
                    if 0 < i && dp[i - 1][j][k] {
                        dp[i][j][k ^ grid[i][j] as usize] = true;
                    }
                    if 0 < j && dp[i][j - 1][k] {
                        dp[i][j][k ^ grid[i][j] as usize] = true;
                    }
                }
            }
        }

        (0..dp[0][0].len())
            .find(|k| dp[dp.len() - 1][dp[0].len() - 1][*k])
            .unwrap() as i32
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_3882() {
        assert_eq!(Solution::min_cost(vec![vec![1, 2], vec![3, 4]]), 6);
        assert_eq!(Solution::min_cost(vec![vec![6, 7], vec![5, 8]]), 9);
        assert_eq!(Solution::min_cost(vec![vec![2, 7, 5]]), 0);
    }
}

fn main() {}
