pub struct Solution {}

impl Solution {
    pub fn count_paths(grid: Vec<Vec<i32>>) -> i32 {
        const M: usize = 1_000_000_007;
        let mut values = Vec::with_capacity(grid.len() * grid[0].len());
        for i in 0..grid.len() {
            for j in 0..grid[0].len() {
                values.push((grid[i][j], i as i32, j as i32));
            }
        }
        values.sort_unstable();
        let mut dp = vec![vec![1; grid[0].len()]; grid.len()];
        let mut sum = 0;

        for i in 0..values.len() {
            sum += dp[values[i].1 as usize][values[i].2 as usize];
            sum %= M;
            for (dr, dc) in [(-1, 0), (0, -1), (0, 1), (1, 0)].iter() {
                let (r, c) = (values[i].1 + dr, values[i].2 + dc);
                if r < 0
                    || c < 0
                    || grid.len() as i32 <= r
                    || grid[0].len() as i32 <= c
                    || grid[r as usize][c as usize] <= values[i].0
                {
                    continue;
                }
                dp[r as usize][c as usize] += dp[values[i].1 as usize][values[i].2 as usize];
                dp[r as usize][c as usize] %= M;
            }
        }

        sum as i32
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_2328() {
        assert_eq!(Solution::count_paths(vec![vec![1, 1], vec![3, 4]]), 8);
        assert_eq!(Solution::count_paths(vec![vec![1], vec![2]]), 3);
    }
}
