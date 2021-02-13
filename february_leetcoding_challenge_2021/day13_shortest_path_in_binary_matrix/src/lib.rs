use std::collections::HashSet;

pub struct Solution {}

impl Solution {
    pub fn shortest_path_binary_matrix(grid: Vec<Vec<i32>>) -> i32 {
        if grid.is_empty() {
            return -1;
        }

        let mut seen = HashSet::new();
        let mut cells = HashSet::new();
        let mut count = 0;
        let goal = (grid.len() - 1, grid[0].len() - 1);
        if grid[0][0] == 0 {
            cells.insert((0, 0));
        }

        while !cells.is_empty() {
            count += 1;
            let mut next_cells = HashSet::new();
            for c in cells {
                if seen.contains(&c) {
                    continue;
                }
                if c == goal {
                    return count;
                }
                seen.insert(c);
                for d in [
                    (-1, -1),
                    (-1, 0),
                    (-1, 1),
                    (0, -1),
                    (0, 1),
                    (1, -1),
                    (1, 0),
                    (1, 1),
                ]
                .iter()
                {
                    let (x, y) = (c.0 as i32 + d.0, c.1 as i32 + d.1);
                    if x < 0
                        || grid.len() as i32 - 1 < x
                        || y < 0
                        || grid[0].len() as i32 - 1 < y
                        || grid[x as usize][y as usize] == 1
                    {
                        continue;
                    }
                    next_cells.insert((x as usize, y as usize));
                }
            }
            cells = next_cells;
        }
        -1
    }
}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn test_day13() {
        assert_eq!(Solution::shortest_path_binary_matrix(vec![]), -1);
        assert_eq!(
            Solution::shortest_path_binary_matrix(vec![vec![0, 1], vec![1, 1]]),
            -1
        );
        assert_eq!(
            Solution::shortest_path_binary_matrix(vec![vec![0, 1], vec![1, 0]]),
            2
        );
        assert_eq!(
            Solution::shortest_path_binary_matrix(vec![
                vec![0, 0, 0],
                vec![1, 1, 0],
                vec![1, 1, 0]
            ]),
            4
        );
    }
}
