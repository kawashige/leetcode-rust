use std::{cmp::Reverse, collections::BinaryHeap};

pub struct Solution {}

impl Solution {
    pub fn minimum_obstacles(grid: Vec<Vec<i32>>) -> i32 {
        let mut min_removal = vec![vec![std::i32::MAX; grid[0].len()]; grid.len()];
        let mut heap = BinaryHeap::new();
        heap.push((Reverse(0), (0, 0)));

        while let Some((Reverse(r), (i, j))) = heap.pop() {
            if min_removal[i][j] <= r {
                continue;
            }
            min_removal[i][j] = r;

            for (di, dj) in [(-1, 0), (0, -1), (0, 1), (1, 0)].iter() {
                let (new_i, new_j) = (i as i32 + di, j as i32 + dj);
                if !(0..grid.len() as i32).contains(&new_i)
                    || !(0..grid[0].len() as i32).contains(&new_j)
                {
                    continue;
                }
                heap.push((
                    Reverse(r + grid[new_i as usize][new_j as usize]),
                    (new_i as usize, new_j as usize),
                ));
            }
        }

        min_removal[grid.len() - 1][grid[0].len() - 1]
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_2290() {
        assert_eq!(
            Solution::minimum_obstacles(vec![vec![0, 1, 1], vec![1, 1, 0], vec![1, 1, 0]]),
            2
        );
        assert_eq!(
            Solution::minimum_obstacles(vec![
                vec![0, 1, 0, 0, 0],
                vec![0, 1, 0, 1, 0],
                vec![0, 0, 0, 1, 0]
            ]),
            0
        );
    }
}
