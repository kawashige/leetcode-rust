pub struct Solution {}

impl Solution {
    pub fn unique_paths_with_obstacles(obstacle_grid: Vec<Vec<i32>>) -> i32 {
        use std::collections::HashMap;

        fn find_paths(m: i32, n: i32, grid: Vec<Vec<i32>>, memo: &mut HashMap<String, i32>) -> i32 {
            let max_m = grid.len() as i32;
            let max_n = grid[0].len() as i32;
            if m == max_m - 1 && n == max_n - 1 {
                match grid[m as usize][n as usize] {
                    0 => return 1,
                    _ => return 0,
                }
            }

            let key = format!("{},{}", m, n);
            match memo.get(&key) {
                Some(n) => *n,
                None => {
                    let mut paths = 0;
                    if m + 1 < max_m && grid[m as usize + 1][n as usize] == 0 {
                        paths += find_paths(m + 1, n, grid.clone(), memo);
                    }
                    if n + 1 < max_n && grid[m as usize][n as usize + 1] == 0 {
                        paths += find_paths(m, n + 1, grid, memo);
                    }
                    memo.insert(key, paths);
                    paths
                }
            }
        }

        match obstacle_grid[0][0] {
            1 => 0,
            _ => find_paths(0, 0, obstacle_grid, &mut HashMap::new()),
        }
    }
}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn test_0063() {
        let input = vec![vec![0, 0, 0], vec![0, 1, 0], vec![0, 0, 0]];
        assert_eq!(2, Solution::unique_paths_with_obstacles(input));
        assert_eq!(1, Solution::unique_paths_with_obstacles(vec![vec![0]]));
        assert_eq!(0, Solution::unique_paths_with_obstacles(vec![vec![1]]));
        assert_eq!(0, Solution::unique_paths_with_obstacles(vec![vec![1, 0]]));
        assert_eq!(0, Solution::unique_paths_with_obstacles(vec![vec![0, 1]]));
    }
}
