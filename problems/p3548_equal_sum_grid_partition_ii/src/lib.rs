pub struct Solution {}

impl Solution {
    pub fn can_partition_grid(grid: Vec<Vec<i32>>) -> bool {
        let mut sum = 0;
        let mut count = vec![0; 100_001];

        for i in 0..grid.len() {
            for j in 0..grid[0].len() {
                sum += grid[i][j] as i64;
                count[grid[i][j] as usize] += 1;
            }
        }

        // horizontal
        let mut h_count = vec![0; 100_001];
        let mut h_sum = 0;
        for i in 0..grid.len() {
            for j in 0..grid[0].len() {
                h_count[grid[i][j] as usize] += 1;
                h_sum += grid[i][j] as i64;
            }
            if sum - h_sum == h_sum {
                return true;
            }
            let d = (h_sum - (sum - h_sum)).abs();
            if h_count.len() < d as usize {
                continue;
            }
            if grid[0].len() == 1 {
                if (sum - h_sum < h_sum && (grid[0][0] as i64 == d || grid[i][0] as i64 == d))
                    || (sum - h_sum > h_sum
                        && (grid[i + 1][0] as i64 == d || grid[grid.len() - 1][0] as i64 == d))
                {
                    return true;
                }
            } else {
                if sum - h_sum < h_sum {
                    if (i == 0
                        && (grid[0][0] as i64 == d || grid[0][grid[0].len() - 1] as i64 == d))
                        || (i != 0 && 0 < h_count[d as usize])
                    {
                        return true;
                    }
                } else {
                    if (i + 2 == grid.len()
                        && (grid[grid.len() - 1][0] as i64 == d
                            || grid[grid.len() - 1][grid[0].len() - 1] as i64 == d))
                        || (i + 2 != grid.len() && 0 < count[d as usize] - h_count[d as usize])
                    {
                        return true;
                    }
                }
            }
        }

        // vertical
        let mut v_count = vec![0; 100_001];
        let mut v_sum = 0;
        for j in 0..grid[0].len() {
            for i in 0..grid.len() {
                v_count[grid[i][j] as usize] += 1;
                v_sum += grid[i][j] as i64;
            }
            if sum - v_sum == v_sum {
                return true;
            }
            let d = (v_sum - (sum - v_sum)).abs();
            if v_count.len() < d as usize {
                continue;
            }
            if grid.len() == 1 {
                if (sum - v_sum < v_sum && (grid[0][0] as i64 == d || grid[0][j] as i64 == d))
                    || (sum - v_sum > v_sum
                        && (grid[0][j + 1] as i64 == d || grid[0][grid[0].len() - 1] as i64 == d))
                {
                    return true;
                }
            } else {
                if sum - v_sum < v_sum {
                    if (j == 0 && (grid[0][0] as i64 == d || grid[grid.len() - 1][0] as i64 == d))
                        || (j != 0 && 0 < v_count[d as usize])
                    {
                        return true;
                    }
                } else {
                    if (j + 2 == grid[0].len()
                        && (grid[0][grid[0].len() - 1] as i64 == d
                            || grid[grid.len() - 1][grid[0].len() - 1] as i64 == d))
                        || (j + 2 != grid[0].len() && 0 < count[d as usize] - v_count[d as usize])
                    {
                        return true;
                    }
                }
            }
        }

        false
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_3548() {
        assert!(Solution::can_partition_grid(vec![vec![1, 4], vec![2, 3]]));
        assert!(Solution::can_partition_grid(vec![vec![1, 2], vec![3, 4]]));
        assert!(!Solution::can_partition_grid(vec![
            vec![1, 2, 4],
            vec![2, 3, 5]
        ]));
        assert!(!Solution::can_partition_grid(vec![
            vec![4, 1, 8],
            vec![3, 2, 6]
        ]));
    }
}
