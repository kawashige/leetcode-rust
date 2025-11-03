use std::collections::VecDeque;

pub struct Solution {}

impl Solution {
    pub fn find_safe_walk(grid: Vec<Vec<i32>>, health: i32) -> bool {
        let mut seen = vec![vec![0; grid[0].len()]; grid.len()];

        let mut queue = VecDeque::new();
        queue.push_back(((0, 0), health - grid[0][0]));

        while let Some(((i, j), h)) = queue.pop_front() {
            if (i, j) == (grid.len() - 1, grid[0].len() - 1) {
                return true;
            }
            if h <= seen[i][j] {
                continue;
            }
            seen[i][j] = h;
            for (di, dj) in [(-1, 0), (0, -1), (0, 1), (1, 0)].iter() {
                let (new_i, new_j) = (i as i32 + di, j as i32 + dj);
                if !(0..grid.len() as i32).contains(&new_i)
                    || !(0..grid[0].len() as i32).contains(&new_j)
                    || (h == 1 && grid[new_i as usize][new_j as usize] == 1)
                {
                    continue;
                }
                queue.push_back((
                    (new_i as usize, new_j as usize),
                    h - grid[new_i as usize][new_j as usize],
                ));
            }
        }

        false
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_3286() {
        assert!(Solution::find_safe_walk(
            vec![
                vec![0, 1, 0, 0, 0],
                vec![0, 1, 0, 1, 0],
                vec![0, 0, 0, 1, 0]
            ],
            1
        ));
        assert!(!Solution::find_safe_walk(
            vec![
                vec![0, 1, 1, 0, 0, 0],
                vec![1, 0, 1, 0, 0, 0],
                vec![0, 1, 1, 1, 0, 1],
                vec![0, 0, 1, 0, 1, 0]
            ],
            3
        ));
        assert!(Solution::find_safe_walk(
            vec![vec![1, 1, 1], vec![1, 0, 1], vec![1, 1, 1]],
            5
        ));
    }
}
