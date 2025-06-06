pub struct Solution {}

impl Solution {
    pub fn minimum_operations(grid: Vec<Vec<i32>>) -> i32 {
        let mut dp = vec![vec![0; 10]; grid[0].len()];

        let mut count = vec![vec![0; 10]; grid[0].len()];
        for i in 0..grid.len() {
            for j in 0..grid[0].len() {
                count[j][grid[i][j] as usize] += 1;
            }
        }
        for i in 0..10 {
            dp[0][i] = grid.len() - count[0][i];
        }

        for i in 1..grid[0].len() {
            for cur in 0..10 {
                let mut min = std::usize::MAX;
                for prev in 0..10 {
                    if cur != prev {
                        min = min.min(dp[i - 1][prev]);
                    }
                }
                dp[i][cur] = min + grid.len() - count[i][cur];
            }
        }

        dp.into_iter().last().unwrap().into_iter().min().unwrap() as i32
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_3122() {
        assert_eq!(
            Solution::minimum_operations(vec![vec![1, 0, 2], vec![1, 0, 2]]),
            0
        );
        assert_eq!(
            Solution::minimum_operations(vec![vec![1, 1, 1], vec![0, 0, 0]]),
            3
        );
        assert_eq!(
            Solution::minimum_operations(vec![vec![1], vec![2], vec![3]]),
            2
        );
    }
}
