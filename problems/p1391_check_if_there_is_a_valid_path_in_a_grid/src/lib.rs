pub struct Solution {}

impl Solution {
    pub fn is_connected(
        r1: usize,
        c1: usize,
        r2: usize,
        c2: usize,
        dr: i32,
        dc: i32,
        grid: &Vec<Vec<i32>>,
    ) -> bool {
        match (dr, dc) {
            (-1, 0) => Self::has_up(r1, c1, &grid) && Self::has_down(r2, c2, &grid),
            (0, -1) => Self::has_left(r1, c1, &grid) && Self::has_right(r2, c2, &grid),
            (0, 1) => Self::has_right(r1, c1, &grid) && Self::has_left(r2, c2, &grid),
            (1, 0) => Self::has_down(r1, c1, &grid) && Self::has_up(r2, c2, &grid),
            _ => unreachable!(),
        }
    }

    pub fn has_up(r: usize, c: usize, grid: &Vec<Vec<i32>>) -> bool {
        [2, 5, 6].contains(&grid[r][c])
    }

    pub fn has_down(r: usize, c: usize, grid: &Vec<Vec<i32>>) -> bool {
        [2, 3, 4].contains(&grid[r][c])
    }

    pub fn has_right(r: usize, c: usize, grid: &Vec<Vec<i32>>) -> bool {
        [1, 4, 6].contains(&grid[r][c])
    }

    pub fn has_left(r: usize, c: usize, grid: &Vec<Vec<i32>>) -> bool {
        [1, 3, 5].contains(&grid[r][c])
    }

    pub fn has_valid_path(grid: Vec<Vec<i32>>) -> bool {
        let mut seen = vec![vec![false; grid[0].len()]; grid.len()];
        let mut stack = vec![(0, 0)];

        while let Some((r, c)) = stack.pop() {
            if seen[r][c] {
                continue;
            }
            if r == grid.len() - 1 && c == grid[0].len() - 1 {
                return true;
            }
            seen[r][c] = true;

            for (dr, dc) in [(-1, 0), (0, -1), (0, 1), (1, 0)].iter() {
                let (new_r, new_c) = (r as i32 + dr, c as i32 + dc);
                if new_r < 0
                    || grid.len() as i32 <= new_r
                    || new_c < 0
                    || grid[0].len() as i32 <= new_c
                    || seen[new_r as usize][new_c as usize]
                    || !Self::is_connected(r, c, new_r as usize, new_c as usize, *dr, *dc, &grid)
                {
                    continue;
                }
                stack.push((new_r as usize, new_c as usize));
            }
        }

        false
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_1391() {
        assert!(Solution::has_valid_path(vec![vec![2, 4, 3], vec![6, 5, 2]]));
        assert!(!Solution::has_valid_path(vec![
            vec![1, 2, 1],
            vec![1, 2, 1]
        ]));
        assert!(!Solution::has_valid_path(vec![vec![1, 1, 2]]));
    }
}
