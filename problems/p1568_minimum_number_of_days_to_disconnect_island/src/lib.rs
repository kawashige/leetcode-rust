pub struct Solution {}

impl Solution {
    pub fn island_count(grid: &Vec<Vec<i32>>) -> usize {
        let mut seen = vec![vec![false; grid[0].len()]; grid.len()];
        let mut count = 0;
        for i in 0..grid.len() {
            for j in 0..grid[0].len() {
                if seen[i][j] || grid[i][j] == 0 {
                    continue;
                }
                count += 1;
                let mut stack = vec![(i, j)];
                while let Some((i, j)) = stack.pop() {
                    if seen[i][j] {
                        continue;
                    }
                    seen[i][j] = true;
                    for (di, dj) in [(-1, 0), (0, -1), (0, 1), (1, 0)].iter() {
                        let (new_i, new_j) = (i as i32 + di, j as i32 + dj);
                        if !(0..grid.len() as i32).contains(&new_i)
                            || !(0..grid[0].len() as i32).contains(&new_j)
                            || seen[new_i as usize][new_j as usize]
                            || grid[new_i as usize][new_j as usize] == 0
                        {
                            continue;
                        }
                        stack.push((new_i as usize, new_j as usize));
                    }
                }
            }
        }
        count
    }

    pub fn min_days(grid: Vec<Vec<i32>>) -> i32 {
        if Self::island_count(&grid) != 1 {
            return 0;
        }

        for i in 0..grid.len() {
            for j in 0..grid[0].len() {
                if grid[i][j] == 1 {
                    let mut new_grid = grid.clone();
                    new_grid[i][j] = 0;
                    if Self::island_count(&new_grid) != 1 {
                        return 1;
                    }
                }
            }
        }

        2
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_1580() {
        assert_eq!(
            Solution::min_days(vec![vec![0, 1, 1, 0], vec![0, 1, 1, 0], vec![0, 0, 0, 0]]),
            2
        );
        assert_eq!(Solution::min_days(vec![vec![1, 1]]), 2);
    }
}
