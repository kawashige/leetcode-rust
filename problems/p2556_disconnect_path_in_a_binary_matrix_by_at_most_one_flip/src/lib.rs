pub struct Solution {}

impl Solution {
    pub fn recurse(
        pos: (usize, usize),
        grid: &Vec<Vec<i32>>,
        seen: &mut Vec<Vec<bool>>,
        right_first: bool,
    ) -> bool {
        if pos == (grid.len() - 1, grid[0].len() - 1) {
            return true;
        }

        for (di, dj) in if right_first {
            [(1, 0), (0, 1)].iter()
        } else {
            [(0, 1), (1, 0)].iter()
        } {
            let (next_i, next_j) = (pos.0 + di, pos.1 + dj);
            if next_i == grid.len() || next_j == grid[0].len() || grid[next_i][next_j] == 0 {
                continue;
            }
            seen[next_i][next_j] = true;
            if Self::recurse((next_i, next_j), grid, seen, right_first) {
                return true;
            } else {
                seen[next_i][next_j] = false;
            }
        }

        false
    }

    pub fn is_possible_to_cut_path(grid: Vec<Vec<i32>>) -> bool {
        if grid.len() * grid[0].len() < 3 {
            return false;
        }

        let mut seen1 = vec![vec![false; grid[0].len()]; grid.len()];
        let mut seen2 = vec![vec![false; grid[0].len()]; grid.len()];

        Self::recurse((0, 0), &grid, &mut seen1, true);
        Self::recurse((0, 0), &grid, &mut seen2, false);

        if !seen1[seen1.len() - 1][seen1[0].len() - 1] {
            return true;
        }

        for i in 0..seen1.len() {
            for j in 0..seen1[0].len() {
                if (i, j) != (seen1.len() - 1, seen1[0].len() - 1) && seen1[i][j] && seen2[i][j] {
                    return true;
                }
            }
        }

        false
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_2556() {
        assert!(!Solution::is_possible_to_cut_path(vec![vec![1]]));
        assert!(Solution::is_possible_to_cut_path(vec![
            vec![1, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0],
            vec![1, 1, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0],
            vec![1, 1, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0],
            vec![1, 1, 1, 1, 1, 1, 1, 0, 0, 0, 0, 0],
            vec![0, 0, 0, 1, 1, 1, 1, 1, 1, 0, 0, 0],
            vec![0, 0, 0, 0, 1, 1, 1, 1, 1, 0, 0, 0],
            vec![0, 0, 0, 0, 1, 1, 1, 1, 1, 0, 0, 0],
            vec![0, 0, 0, 0, 0, 0, 1, 1, 1, 1, 1, 1],
            vec![0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 1],
            vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 1],
            vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1],
            vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1],
            vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1],
            vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1],
            vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1],
            vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1],
            vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1]
        ]));
        assert!(Solution::is_possible_to_cut_path(vec![
            vec![1, 1, 1],
            vec![1, 0, 0],
            vec![1, 1, 1]
        ]));
        assert!(!Solution::is_possible_to_cut_path(vec![
            vec![1, 1, 1],
            vec![1, 0, 1],
            vec![1, 1, 1]
        ]));
    }
}
