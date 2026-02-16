pub struct Solution {}

impl Solution {
    pub fn can_partition_grid(grid: Vec<Vec<i32>>) -> bool {
        let mut row_sum = vec![0; grid.len()];
        let mut column_sum = vec![0; grid[0].len()];
        let mut sum = 0;

        for i in 0..grid.len() {
            for j in 0..grid[0].len() {
                sum += grid[i][j] as usize;
                row_sum[i] += grid[i][j] as usize;
                column_sum[j] += grid[i][j] as usize;
            }
        }

        let mut tmp_sum = 0;
        for i in 0..grid.len() - 1 {
            tmp_sum += row_sum[i];
            if sum - tmp_sum == tmp_sum {
                return true;
            }
        }
        let mut tmp_sum = 0;
        for i in 0..grid[0].len() - 1 {
            tmp_sum += column_sum[i];
            if sum - tmp_sum == tmp_sum {
                return true;
            }
        }

        false
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_3546() {
        assert!(Solution::can_partition_grid(vec![vec![1, 4], vec![2, 3]]));
        assert!(!Solution::can_partition_grid(vec![vec![1, 3], vec![2, 4]]));
    }
}
