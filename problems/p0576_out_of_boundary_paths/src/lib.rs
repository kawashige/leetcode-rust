pub struct Solution {}

impl Solution {
    pub fn find_paths(m: i32, n: i32, num: i32, i: i32, j: i32) -> i32 {
        let modulo = 1_000_000_007;
        let mut grid = vec![vec![0; n as usize]; m as usize];
        grid[i as usize][j as usize] = 1;

        let mut result: u64 = 0;
        for _ in 0..num {
            let mut new_grid = vec![vec![0; n as usize]; m as usize];
            for k in 0..grid.len() {
                for l in 0..grid[0].len() {
                    if grid[k][l] == 0 {
                        continue;
                    }
                    if k == 0 {
                        result = (result + grid[k][l]) % modulo;
                    } else {
                        new_grid[k - 1][l] = (new_grid[k - 1][l] + grid[k][l]) % modulo;
                    }
                    if k == grid.len() - 1 {
                        result = (result + grid[k][l]) % modulo;
                    } else {
                        new_grid[k + 1][l] = (new_grid[k + 1][l] + grid[k][l]) % modulo;
                    }
                    if l == 0 {
                        result = (result + grid[k][l]) % modulo;
                    } else {
                        new_grid[k][l - 1] = (new_grid[k][l - 1] + grid[k][l]) % modulo;
                    }
                    if l == grid[0].len() - 1 {
                        result = (result + grid[k][l]) % modulo;
                    } else {
                        new_grid[k][l + 1] = (new_grid[k][l + 1] + grid[k][l]) % modulo;
                    }
                }
            }
            grid = new_grid;
        }
        result as i32
    }
}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn test_0576() {
        assert_eq!(Solution::find_paths(36, 5, 50, 15, 3), 390153306);
        assert_eq!(Solution::find_paths(8, 50, 23, 5, 26), 914783380);
        assert_eq!(Solution::find_paths(2, 2, 2, 0, 0), 6);
        assert_eq!(Solution::find_paths(1, 3, 3, 0, 1), 12);
    }
}
