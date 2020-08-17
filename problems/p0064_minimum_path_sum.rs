pub struct Solution {}

impl Solution {
    pub fn min_path_sum(grid: Vec<Vec<i32>>) -> i32 {
        use std::collections::HashMap;

        fn find_paths(m: i32, n: i32, grid: Vec<Vec<i32>>, memo: &mut HashMap<String, i32>) -> i32 {
            if m == 0 && n == 0 {
                return grid[0][0];
            }

            let key = format!("{},{}", m, n);
            match memo.get(&key) {
                Some(n) => *n,
                None => {
                    let mut paths = Vec::new();
                    if m - 1 >= 0 {
                        paths.push(
                            grid[m as usize][n as usize] + find_paths(m - 1, n, grid.clone(), memo),
                        );
                    }
                    if n - 1 >= 0 {
                        paths.push(grid[m as usize][n as usize] + find_paths(m, n - 1, grid, memo));
                    }
                    let min_path = paths.iter().min().unwrap();
                    memo.insert(key, *min_path);
                    *min_path
                }
            }
        }

        find_paths(
            grid.len() as i32 - 1,
            grid[0].len() as i32 - 1,
            grid,
            &mut HashMap::new(),
        )
    }
}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn test_0064() {
        let input = vec![vec![1, 3, 1], vec![1, 5, 1], vec![4, 2, 1]];

        assert_eq!(7, Solution::min_path_sum(input));
        assert_eq!(4, Solution::min_path_sum(vec![vec![4]]));
    }
}
