pub struct Solution {}

impl Solution {
    pub fn number_of_paths(grid: Vec<Vec<i32>>, k: i32) -> i32 {
        const M: usize = 1_000_000_007;
        let k = k as usize;
        let mut count = vec![vec![vec![0; k]; grid[0].len()]; grid.len()];
        count[0][0][grid[0][0] as usize % k] = 1;

        for i in 0..grid.len() {
            for j in 0..grid[0].len() {
                for l in 0..k {
                    if 0 < i {
                        count[i][j][(l + grid[i][j] as usize) % k] += count[i - 1][j][l];
                        count[i][j][(l + grid[i][j] as usize) % k] %= M;
                    }
                    if 0 < j {
                        count[i][j][(l + grid[i][j] as usize) % k] += count[i][j - 1][l];
                        count[i][j][(l + grid[i][j] as usize) % k] %= M;
                    }
                }
            }
        }

        count[grid.len() - 1][grid[0].len() - 1][0] as i32
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_2430() {
        assert_eq!(
            Solution::number_of_paths(vec![vec![5, 2, 4], vec![3, 0, 5], vec![0, 7, 2]], 3),
            2
        );
        assert_eq!(Solution::number_of_paths(vec![vec![0, 0]], 5), 1);
        assert_eq!(
            Solution::number_of_paths(
                vec![vec![7, 3, 4, 9], vec![2, 3, 6, 2], vec![2, 3, 7, 0]],
                1
            ),
            10
        );
    }
}
