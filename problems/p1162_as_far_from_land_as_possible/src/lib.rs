use std::collections::VecDeque;

pub struct Solution {}

impl Solution {
    pub fn max_distance(grid: Vec<Vec<i32>>) -> i32 {
        let mut queue = VecDeque::new();
        let mut dist = vec![vec![0; grid[0].len()]; grid.len()];

        for i in 0..grid.len() {
            for j in 0..grid[0].len() {
                if grid[i][j] == 1 {
                    queue.push_back(((i, j), 0));
                }
            }
        }

        let mut max = 0;

        while let Some(((i, j), c)) = queue.pop_front() {
            for (dx, dy) in [(-1, 0), (0, -1), (0, 1), (1, 0)].iter() {
                let (x, y) = (i as i32 + dx, j as i32 + dy);
                if x < 0
                    || grid.len() as i32 <= x
                    || y < 0
                    || grid[0].len() as i32 <= y
                    || 0 < dist[x as usize][y as usize]
                    || grid[x as usize][y as usize] == 1
                {
                    continue;
                }
                dist[x as usize][y as usize] = c + 1;
                max = max.max(c + 1);
                queue.push_back(((x as usize, y as usize), c + 1));
            }
        }

        if max == 0 {
            -1
        } else {
            max
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_1162() {
        assert_eq!(
            Solution::max_distance(vec![
                vec![1, 0, 0, 0, 0, 1, 0, 0, 0, 1],
                vec![1, 1, 0, 1, 1, 1, 0, 1, 1, 0],
                vec![0, 1, 1, 0, 1, 0, 0, 1, 0, 0],
                vec![1, 0, 1, 0, 1, 0, 0, 0, 0, 0],
                vec![0, 1, 0, 0, 0, 1, 1, 0, 1, 1],
                vec![0, 0, 1, 0, 0, 1, 0, 1, 0, 1],
                vec![0, 0, 0, 1, 1, 1, 1, 0, 0, 1],
                vec![0, 1, 0, 0, 1, 0, 0, 1, 0, 0],
                vec![0, 0, 0, 0, 0, 1, 1, 1, 0, 0],
                vec![1, 1, 0, 1, 1, 1, 1, 1, 0, 0]
            ]),
            2
        );
        assert_eq!(
            Solution::max_distance(vec![vec![1, 0, 1], vec![0, 0, 0], vec![1, 0, 1]]),
            2
        );
        assert_eq!(
            Solution::max_distance(vec![vec![1, 0, 0], vec![0, 0, 0], vec![0, 0, 0]]),
            4
        );
    }
}
