pub struct Solution {}

impl Solution {
    pub fn count_submatrices(grid: Vec<Vec<i32>>, k: i32) -> i32 {
        let mut sum = vec![vec![0; grid[0].len()]; grid.len()];
        let mut result = 0;
        for i in 0..grid.len() {
            for j in 0..grid[0].len() {
                sum[i][j] = grid[i][j];
                if 0 < i {
                    sum[i][j] += sum[i - 1][j];
                }
                if 0 < j {
                    sum[i][j] += sum[i][j - 1];
                }
                if 0 < i && 0 < j {
                    sum[i][j] -= sum[i - 1][j - 1];
                }
                if sum[i][j] <= k {
                    result += 1;
                }
            }
        }
        result
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_3070() {
        assert_eq!(
            Solution::count_submatrices(vec![vec![7, 6, 3], vec![6, 6, 1]], 18),
            4
        );
        assert_eq!(
            Solution::count_submatrices(vec![vec![7, 2, 9], vec![1, 5, 0], vec![2, 6, 6]], 20),
            6
        );
    }
}
