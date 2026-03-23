pub struct Solution {}

impl Solution {
    pub fn count_islands(grid: Vec<Vec<i32>>, k: i32) -> i32 {
        let mut grid = grid;
        let mut result = 0;

        for i in 0..grid.len() {
            for j in 0..grid[0].len() {
                if grid[i][j] == 0 {
                    continue;
                }
                let mut sum = 0;
                let mut stack = vec![(i, j)];
                while let Some((x, y)) = stack.pop() {
                    if grid[x][y] == 0 {
                        continue;
                    }
                    sum += grid[x][y] as usize;
                    grid[x][y] = 0;
                    for (dx, dy) in [(-1, 0), (0, -1), (0, 1), (1, 0)].iter() {
                        let (new_x, new_y) = (x as i32 + dx, y as i32 + dy);
                        if !(0..grid.len() as i32).contains(&new_x)
                            || !(0..grid[0].len() as i32).contains(&new_y)
                            || grid[new_x as usize][new_y as usize] == 0
                        {
                            continue;
                        }
                        stack.push((new_x as usize, new_y as usize));
                    }
                }
                if sum % k as usize == 0 {
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
    fn test_3619() {
        assert_eq!(
            Solution::count_islands(
                vec![
                    vec![0, 2, 1, 0, 0],
                    vec![0, 5, 0, 0, 5],
                    vec![0, 0, 1, 0, 0],
                    vec![0, 1, 4, 7, 0],
                    vec![0, 2, 0, 0, 8]
                ],
                5
            ),
            2
        );
        assert_eq!(
            Solution::count_islands(
                vec![vec![3, 0, 3, 0], vec![0, 3, 0, 3], vec![3, 0, 3, 0]],
                3
            ),
            6
        );
    }
}
