pub struct Solution {}

impl Solution {
    pub fn fill(i: usize, j: usize, x: i32, p: i32, grid: &mut Vec<Vec<i32>>) {
        if p == 0 {
            grid[i][j] = x;
            return;
        }

        let d = 2_usize.pow(p as u32 - 1);
        let d2 = 2_i32.pow(p as u32 - 1) * 2_i32.pow(p as u32 - 1);

        Self::fill(i, j, x, p - 1, grid);
        Self::fill(i + d, j, x + d2, p - 1, grid);
        Self::fill(i + d, j - d, x + d2 * 2, p - 1, grid);
        Self::fill(i, j - d, x + d2 * 3, p - 1, grid);
    }

    pub fn special_grid(n: i32) -> Vec<Vec<i32>> {
        let l = 2_usize.pow(n as u32);
        let mut grid = vec![vec![-1; l]; l];

        Self::fill(0, l - 1, 0, n, &mut grid);

        grid
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_3537() {
        assert_eq!(Solution::special_grid(0), vec![vec![0]]);
        assert_eq!(Solution::special_grid(1), vec![vec![3, 0], vec![2, 1]]);
        assert_eq!(
            Solution::special_grid(2),
            vec![
                vec![15, 12, 3, 0],
                vec![14, 13, 2, 1],
                vec![11, 8, 7, 4],
                vec![10, 9, 6, 5]
            ]
        );
    }
}
