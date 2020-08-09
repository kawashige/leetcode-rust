pub struct Solution {}

impl Solution {
    pub fn oranges_rotting(grid: Vec<Vec<i32>>) -> i32 {
        let mut grid_now = grid.clone();
        let mut time = 0;
        loop {
            if grid_now.iter().all(|r| !r.contains(&1)) {
                return time;
            }

            let mut grid_next = grid_now.clone();
            let mut changed = false;
            time += 1;
            for i in 0..grid_now.len() {
                for j in 0..grid_now[i].len() {
                    if grid_now[i][j] == 2 {
                        if i >= 1 && grid_now[i - 1][j] == 1 {
                            grid_next[i - 1][j] = 2;
                            changed = true;
                        }
                        if i + 1 < grid_now.len() && grid_now[i + 1][j] == 1 {
                            grid_next[i + 1][j] = 2;
                            changed = true;
                        }
                        if j >= 1 && grid_now[i][j - 1] == 1 {
                            grid_next[i][j - 1] = 2;
                            changed = true;
                        }
                        if j + 1 < grid_now[i].len() && grid_now[i][j + 1] == 1 {
                            grid_next[i][j + 1] = 2;
                            changed = true;
                        }
                    }
                }
            }
            if !changed {
                return -1;
            }
            grid_now = grid_next;
        }
    }
}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn test_day9() {
        assert_eq!(
            4,
            Solution::oranges_rotting(vec![vec![2, 1, 1], vec![1, 1, 0], vec![0, 1, 1]])
        );
        assert_eq!(
            -1,
            Solution::oranges_rotting(vec![vec![2, 1, 1], vec![0, 1, 1], vec![1, 0, 1]])
        );
        assert_eq!(0, Solution::oranges_rotting(vec![vec![0, 2]]));
        assert_eq!(1, Solution::oranges_rotting(vec![vec![1, 2]]));
    }
}
