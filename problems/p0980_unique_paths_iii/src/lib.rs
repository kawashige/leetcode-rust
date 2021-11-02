pub struct Solution {}

impl Solution {
    pub fn dfs(
        i: usize,
        j: usize,
        grid: &Vec<Vec<i32>>,
        seen: &mut Vec<Vec<bool>>,
        remains: &mut usize,
        count: &mut i32,
    ) {
        if grid[i][j] == 2 {
            if remains == &1 {
                *count += 1;
            }
            return;
        }

        seen[i][j] = true;
        *remains -= 1;

        for (dx, dy) in [(-1, 0), (0, -1), (0, 1), (1, 0)].iter() {
            let (x, y) = (i as i32 + dx, j as i32 + dy);
            if x < 0
                || grid.len() as i32 <= x
                || y < 0
                || grid[0].len() as i32 <= y
                || seen[x as usize][y as usize]
                || grid[x as usize][y as usize] == -1
            {
                continue;
            }
            Self::dfs(x as usize, y as usize, grid, seen, remains, count);
        }

        seen[i][j] = false;
        *remains += 1;
    }

    pub fn unique_paths_iii(grid: Vec<Vec<i32>>) -> i32 {
        let mut count = 0;
        let mut x = 0;
        let mut y = 0;
        let mut remains = grid.len() * grid[0].len();
        for i in 0..grid.len() {
            for j in 0..grid[0].len() {
                if grid[i][j] == 1 {
                    x = i;
                    y = j;
                } else if grid[i][j] == -1 {
                    remains -= 1;
                }
            }
        }

        Self::dfs(
            x,
            y,
            &grid,
            &mut vec![vec![false; grid[0].len()]; grid.len()],
            &mut remains,
            &mut count,
        );

        count
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_0980() {
        assert_eq!(
            Solution::unique_paths_iii(vec![vec![1, 0, 0, 0], vec![0, 0, 0, 0], vec![0, 0, 2, -1]]),
            2
        );
        assert_eq!(
            Solution::unique_paths_iii(vec![vec![1, 0, 0, 0], vec![0, 0, 0, 0], vec![0, 0, 0, 2]]),
            4
        );
    }
}
