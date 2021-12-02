pub struct Solution {}

impl Solution {
    pub fn dfs(
        row: usize,
        col: usize,
        grid: &mut Vec<Vec<i32>>,
        target: i32,
        color: i32,
        seen: &mut Vec<Vec<bool>>,
    ) {
        if seen[row][col] {
            return;
        }
        seen[row][col] = true;

        for (dr, dc) in [(-1, 0), (0, -1), (0, 1), (1, 0)].iter() {
            let (new_row, new_col) = (row as i32 + dr, col as i32 + dc);

            if new_row < 0
                || new_row == grid.len() as i32
                || new_col < 0
                || new_col == grid[0].len() as i32
                || (!seen[new_row as usize][new_col as usize]
                    && grid[new_row as usize][new_col as usize] != target)
            {
                grid[row][col] = color;
            } else if !seen[new_row as usize][new_col as usize] {
                Self::dfs(
                    new_row as usize,
                    new_col as usize,
                    grid,
                    target,
                    color,
                    seen,
                );
            }
        }
    }

    pub fn color_border(mut grid: Vec<Vec<i32>>, row: i32, col: i32, color: i32) -> Vec<Vec<i32>> {
        let target = grid[row as usize][col as usize];
        let mut seen = vec![vec![false; grid[0].len()]; grid.len()];
        Self::dfs(
            row as usize,
            col as usize,
            &mut grid,
            target,
            color,
            &mut seen,
        );
        grid
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_1034() {
        assert_eq!(
            Solution::color_border(vec![vec![1, 1], vec![1, 2]], 0, 0, 3),
            vec![vec![3, 3], vec![3, 2]]
        );
        assert_eq!(
            Solution::color_border(vec![vec![1, 2, 2], vec![2, 3, 2]], 0, 1, 3),
            vec![vec![1, 3, 3], vec![2, 3, 3]]
        );
        assert_eq!(
            Solution::color_border(vec![vec![1, 1, 1], vec![1, 1, 1], vec![1, 1, 1]], 1, 1, 2),
            vec![vec![2, 2, 2], vec![2, 1, 2], vec![2, 2, 2]]
        );
    }
}
