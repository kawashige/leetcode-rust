pub struct Solution {}

impl Solution {
    pub fn count_paths_with_xor_value(grid: Vec<Vec<i32>>, k: i32) -> i32 {
        const M: usize = 1_000_000_007;
        let mut xor_values = vec![vec![vec![0; 16]; grid[0].len()]; grid.len()];
        xor_values[0][0][grid[0][0] as usize] = 1;

        for i in 0..grid.len() {
            for j in 0..grid[0].len() {
                for k in 0..16 {
                    if 0 < i {
                        xor_values[i][j][k ^ grid[i][j] as usize] += xor_values[i - 1][j][k];
                        xor_values[i][j][k ^ grid[i][j] as usize] %= M;
                    }
                    if 0 < j {
                        xor_values[i][j][k ^ grid[i][j] as usize] += xor_values[i][j - 1][k];
                        xor_values[i][j][k ^ grid[i][j] as usize] %= M;
                    }
                }
            }
        }

        xor_values[grid.len() - 1][grid[0].len() - 1][k as usize] as i32
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_3393() {
        assert_eq!(
            Solution::count_paths_with_xor_value(
                vec![vec![2, 1, 5], vec![7, 10, 0], vec![12, 6, 4]],
                11
            ),
            3
        );
        assert_eq!(
            Solution::count_paths_with_xor_value(
                vec![vec![1, 3, 3, 3], vec![0, 3, 3, 2], vec![3, 0, 1, 1]],
                2
            ),
            5
        );
        assert_eq!(
            Solution::count_paths_with_xor_value(
                vec![vec![1, 1, 1, 2], vec![3, 0, 3, 2], vec![3, 0, 2, 2]],
                10
            ),
            0
        );
    }
}
