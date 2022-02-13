pub struct Solution {}

impl Solution {
    pub fn dfs(
        i: usize,
        j: usize,
        grid: &Vec<Vec<i32>>,
        seen: &mut Vec<Vec<bool>>,
        is_corner: &mut bool,
    ) {
        seen[i][j] = true;

        for (dx, dy) in [(-1, 0), (0, -1), (0, 1), (1, 0)].iter() {
            let (x, y) = (i as i32 + dx, j as i32 + dy);
            if x < 0 || grid.len() as i32 <= x || y < 0 || grid[0].len() as i32 <= y {
                *is_corner = true;
                continue;
            }
            if seen[x as usize][y as usize] || grid[x as usize][y as usize] == 1 {
                continue;
            }
            Self::dfs(x as usize, y as usize, grid, seen, is_corner);
        }
    }

    pub fn closed_island(grid: Vec<Vec<i32>>) -> i32 {
        let mut seen = vec![vec![false; grid[0].len()]; grid.len()];
        let mut result = 0;

        for i in 0..grid.len() {
            for j in 0..grid[0].len() {
                if grid[i][j] == 0 && !seen[i][j] {
                    let mut is_corner = false;
                    Self::dfs(i, j, &grid, &mut seen, &mut is_corner);
                    if !is_corner {
                        result += 1;
                    }
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
    fn test_1254() {
        assert_eq!(
            Solution::closed_island(vec![
                vec![1, 1, 1, 1, 1, 1, 1, 0],
                vec![1, 0, 0, 0, 0, 1, 1, 0],
                vec![1, 0, 1, 0, 1, 1, 1, 0],
                vec![1, 0, 0, 0, 0, 1, 0, 1],
                vec![1, 1, 1, 1, 1, 1, 1, 0]
            ]),
            2
        );
        assert_eq!(
            Solution::closed_island(vec![
                vec![0, 0, 1, 0, 0],
                vec![0, 1, 0, 1, 0],
                vec![0, 1, 1, 1, 0]
            ]),
            1
        );
        assert_eq!(
            Solution::closed_island(vec![
                vec![1, 1, 1, 1, 1, 1, 1],
                vec![1, 0, 0, 0, 0, 0, 1],
                vec![1, 0, 1, 1, 1, 0, 1],
                vec![1, 0, 1, 0, 1, 0, 1],
                vec![1, 0, 1, 1, 1, 0, 1],
                vec![1, 0, 0, 0, 0, 0, 1],
                vec![1, 1, 1, 1, 1, 1, 1]
            ]),
            2
        );
    }
}
