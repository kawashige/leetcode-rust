use std::collections::VecDeque;

pub struct Solution {}

impl Solution {
    pub fn nearest_exit(maze: Vec<Vec<char>>, entrance: Vec<i32>) -> i32 {
        let mut queue = VecDeque::new();
        queue.push_back(((entrance[0] as usize, entrance[1] as usize), 0));
        let mut seen = vec![vec![false; maze[0].len()]; maze.len()];

        while let Some(((i, j), c)) = queue.pop_front() {
            if seen[i][j] {
                continue;
            }
            seen[i][j] = true;

            if 0 < c && (i == 0 || j == 0 || i == maze.len() - 1 || j == maze[0].len() - 1) {
                return c;
            }

            for (dx, dy) in [(-1, 0), (0, -1), (0, 1), (1, 0)] {
                let (x, y) = (i as i32 + dx, j as i32 + dy);
                if x < 0
                    || y < 0
                    || x == maze.len() as i32
                    || y == maze[0].len() as i32
                    || maze[x as usize][y as usize] == '+'
                    || seen[x as usize][y as usize]
                {
                    continue;
                }
                queue.push_back(((x as usize, y as usize), c + 1));
            }
        }

        -1
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_1926() {
        assert_eq!(
            Solution::nearest_exit(
                vec![
                    vec!['+', '+', '.', '+'],
                    vec!['.', '.', '.', '+'],
                    vec!['+', '+', '+', '.']
                ],
                vec![1, 2]
            ),
            1
        );
        assert_eq!(
            Solution::nearest_exit(
                vec![
                    vec!['+', '+', '+'],
                    vec!['.', '.', '.'],
                    vec!['+', '+', '+']
                ],
                vec![1, 0]
            ),
            2
        );
        assert_eq!(Solution::nearest_exit(vec![vec!['.', '+']], vec![0, 0]), -1);
    }
}
