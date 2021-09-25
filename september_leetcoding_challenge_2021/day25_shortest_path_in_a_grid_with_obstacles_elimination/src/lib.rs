use std::collections::VecDeque;

pub struct Solution {}

impl Solution {
    pub fn shortest_path(grid: Vec<Vec<i32>>, k: i32) -> i32 {
        let mut queue = VecDeque::new();
        queue.push_back(((0, 0), 0, 0));

        let mut seen = vec![vec![vec![false; k as usize + 1]; grid[0].len()]; grid.len()];

        while let Some(((i, j), o, c)) = queue.pop_front() {
            if (i, j) == (grid.len() - 1, grid[0].len() - 1) {
                return c;
            }

            if seen[i][j][o] {
                continue;
            }
            seen[i][j][o] = true;

            for (dx, dy) in [(-1, 0), (0, -1), (0, 1), (1, 0)].iter() {
                let (x, y) = (i as i32 + dx, j as i32 + dy);

                if x < 0 || y < 0 || x >= grid.len() as i32 || y >= grid[0].len() as i32 {
                    continue;
                }

                let next_o = o + if grid[x as usize][y as usize] == 1 {
                    1
                } else {
                    0
                };
                if x < 0
                    || y < 0
                    || (grid[x as usize][y as usize] == 1 && o == k as usize)
                    || seen[x as usize][y as usize][next_o]
                {
                    continue;
                }
                queue.push_back(((x as usize, y as usize), next_o, c + 1));
            }
        }

        return -1;
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_day25() {
        assert_eq!(
            Solution::shortest_path(
                vec![
                    vec![0, 0, 0],
                    vec![1, 1, 0],
                    vec![0, 0, 0],
                    vec![0, 1, 1],
                    vec![0, 0, 0]
                ],
                1
            ),
            6
        );
        assert_eq!(
            Solution::shortest_path(vec![vec![0, 1, 1], vec![1, 1, 1], vec![1, 0, 0]], 1),
            -1
        );
    }
}
