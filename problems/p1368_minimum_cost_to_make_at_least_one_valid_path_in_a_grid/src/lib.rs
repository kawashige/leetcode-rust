use std::{cmp::Reverse, collections::BinaryHeap};

pub struct Solution {}

impl Solution {
    pub fn min_cost(grid: Vec<Vec<i32>>) -> i32 {
        let mut heap = BinaryHeap::new();
        heap.push((Reverse(0), (0, 0)));
        let mut seen = vec![vec![std::i32::MAX; grid[0].len()]; grid.len()];

        let directions = [(0, 1), (0, -1), (1, 0), (-1, 0)];
        while let Some((Reverse(c), (i, j))) = heap.pop() {
            if (i, j) == (grid.len() - 1, grid[0].len() - 1) {
                return c;
            }
            if seen[i][j] <= c {
                continue;
            }
            seen[i][j] = c;
            for k in 0..directions.len() {
                let (new_i, new_j) = (i as i32 + directions[k].0, j as i32 + directions[k].1);
                if !(0..grid.len() as i32).contains(&new_i)
                    || !(0..grid[0].len() as i32).contains(&new_j)
                {
                    continue;
                }
                heap.push((
                    Reverse(c + if grid[i][j] != k as i32 + 1 { 1 } else { 0 }),
                    (new_i as usize, new_j as usize),
                ));
            }
        }

        unreachable!()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_1368() {
        assert_eq!(
            Solution::min_cost(vec![
                vec![1, 1, 1, 1],
                vec![2, 2, 2, 2],
                vec![1, 1, 1, 1],
                vec![2, 2, 2, 2]
            ]),
            3
        );
        assert_eq!(
            Solution::min_cost(vec![vec![1, 1, 3], vec![3, 2, 2], vec![1, 1, 4]]),
            0
        );
        assert_eq!(Solution::min_cost(vec![vec![1, 2], vec![4, 3]]), 1);
    }
}
