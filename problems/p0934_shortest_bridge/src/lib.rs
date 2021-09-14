use std::collections::VecDeque;

pub struct Solution {}

impl Solution {
    pub fn shortest_bridge(mut grid: Vec<Vec<i32>>) -> i32 {
        let p = grid.iter().flatten().take_while(|x| x == &&0).count();

        let mut queue = VecDeque::new();
        let mut queue2 = VecDeque::new();
        queue.push_back((p / grid.len(), p % grid.len()));
        queue2.push_back(((p / grid.len(), p % grid.len()), 0));

        while let Some((i, j)) = queue.pop_front() {
            if grid[i][j] == 2 {
                continue;
            }
            grid[i][j] = 2;

            for (x, y) in [(-1, 0), (0, -1), (0, 1), (1, 0)].iter() {
                let (k, l) = (i as i32 + x, j as i32 + y);
                if k < 0
                    || l < 0
                    || grid.len() as i32 <= k
                    || grid[0].len() as i32 <= l
                    || grid[k as usize][l as usize] != 1
                {
                    continue;
                }
                queue.push_back((k as usize, l as usize));
                queue2.push_back(((k as usize, l as usize), 0));
            }
        }

        while let Some(((i, j), c)) = queue2.pop_front() {
            for (x, y) in [(-1, 0), (0, -1), (0, 1), (1, 0)].iter() {
                let (k, l) = (i as i32 + x, j as i32 + y);
                if k < 0
                    || l < 0
                    || grid.len() as i32 <= k
                    || grid[0].len() as i32 <= l
                    || grid[k as usize][l as usize] == 2
                {
                    continue;
                }
                if grid[k as usize][l as usize] == 1 {
                    return c as i32;
                }
                grid[k as usize][j as usize] = 2;
                queue2.push_back(((k as usize, l as usize), c + 1));
            }
        }

        unreachable!()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_0934() {
        assert_eq!(Solution::shortest_bridge(vec![vec![0, 1], vec![1, 0]]), 1);
        assert_eq!(
            Solution::shortest_bridge(vec![vec![0, 1, 0], vec![0, 0, 0], vec![0, 0, 1]]),
            2
        );
        assert_eq!(
            Solution::shortest_bridge(vec![
                vec![1, 1, 1, 1, 1],
                vec![1, 0, 0, 0, 1],
                vec![1, 0, 1, 0, 1],
                vec![1, 0, 0, 0, 1],
                vec![1, 1, 1, 1, 1]
            ]),
            1
        );

        assert_eq!(
            Solution::shortest_bridge(vec![
                vec![0, 1, 0, 0, 0],
                vec![0, 1, 0, 1, 1],
                vec![0, 0, 0, 0, 1],
                vec![0, 0, 0, 0, 0],
                vec![0, 0, 0, 0, 0]
            ]),
            1
        );
    }
}
