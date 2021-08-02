pub struct Solution {}

impl Solution {
    pub fn surface_area(grid: Vec<Vec<i32>>) -> i32 {
        let mut area = 0;

        for i in 0..grid.len() {
            for j in 0..grid[0].len() {
                area += if grid[i][j] == 0 { 0 } else { 2 };
                area += (grid[i][j] - if i == 0 { 0 } else { grid[i - 1][j] }).abs();
                area += (grid[i][j] - if j == 0 { 0 } else { grid[i][j - 1] }).abs();
                if i == grid.len() - 1 {
                    area += grid[i][j];
                }
                if j == grid[0].len() - 1 {
                    area += grid[i][j];
                }
            }
        }

        area
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_0892() {
        assert_eq!(Solution::surface_area(vec![vec![2]]), 10);
        assert_eq!(Solution::surface_area(vec![vec![1, 2], vec![3, 4]]), 34);
        assert_eq!(Solution::surface_area(vec![vec![1, 0], vec![0, 2]]), 16);
        assert_eq!(
            Solution::surface_area(vec![vec![1, 1, 1], vec![1, 0, 1], vec![1, 1, 1]]),
            32
        );
        assert_eq!(
            Solution::surface_area(vec![vec![2, 2, 2], vec![2, 1, 2], vec![2, 2, 2]]),
            46
        );
    }
}
