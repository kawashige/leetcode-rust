use std::{cmp::Reverse, collections::BinaryHeap};

pub struct Solution {}

impl Solution {
    pub fn minimum_time(grid: Vec<Vec<i32>>) -> i32 {
        if 1 < grid[0][1] && 1 < grid[1][0] {
            return -1;
        }

        let mut heap = BinaryHeap::new();
        heap.push((
            Reverse(grid[0][1] + if grid[0][1] % 2 == 1 { 0 } else { 1 }),
            (0, 1),
        ));
        heap.push((
            Reverse(grid[1][0] + if grid[1][0] % 2 == 1 { 0 } else { 1 }),
            (1, 0),
        ));

        let mut seen = vec![vec![std::i32::MAX; grid[0].len()]; grid.len()];
        seen[0][0] = 0;

        while let Some((Reverse(t), (i, j))) = heap.pop() {
            if seen[i][j] <= t {
                continue;
            }
            seen[i][j] = t;

            for (di, dj) in [(-1, 0), (0, -1), (0, 1), (1, 0)].iter() {
                let (new_i, new_j) = (i as i32 + di, j as i32 + dj);
                if !(0..grid.len() as i32).contains(&new_i)
                    || !(0..grid[0].len() as i32).contains(&new_j)
                {
                    continue;
                }
                let new_t = if grid[new_i as usize][new_j as usize] <= t + 1 {
                    t + 1
                } else {
                    grid[new_i as usize][new_j as usize]
                        + (grid[new_i as usize][new_j as usize] - t - 1) % 2
                };
                heap.push((Reverse(new_t), (new_i as usize, new_j as usize)));
            }
        }

        seen[seen.len() - 1][seen[0].len() - 1]
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_2577() {
        assert_eq!(
            Solution::minimum_time(vec![vec![0, 1, 3, 2], vec![5, 1, 2, 5], vec![4, 3, 8, 6]]),
            7
        );
        assert_eq!(
            Solution::minimum_time(vec![vec![0, 2, 4], vec![3, 2, 1], vec![1, 0, 4]]),
            -1
        );
    }
}
