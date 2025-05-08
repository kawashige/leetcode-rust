use std::{cmp::Reverse, collections::BinaryHeap};

pub struct Solution {}

impl Solution {
    pub fn min_time_to_reach(move_time: Vec<Vec<i32>>) -> i32 {
        let mut seen = vec![vec![vec![std::i32::MAX; 2]; move_time[0].len()]; move_time.len()];
        let mut heap = BinaryHeap::new();
        heap.push((Reverse(0), (0, 0), 0));
        while let Some((Reverse(t), (i, j), s)) = heap.pop() {
            if seen[i][j][s] <= t || (s == 1 && seen[i][j][0] <= t - 1) {
                continue;
            }
            seen[i][j][s] = t;
            for (di, dj) in [(-1, 0), (0, -1), (0, 1), (1, 0)].iter() {
                let (new_i, new_j) = (i as i32 + di, j as i32 + dj);
                if !(0..move_time.len() as i32).contains(&new_i)
                    || !(0..move_time[0].len() as i32).contains(&new_j)
                {
                    continue;
                }
                let new_t = t.max(move_time[new_i as usize][new_j as usize]) + s as i32 + 1;
                heap.push((
                    Reverse(new_t),
                    (new_i as usize, new_j as usize),
                    (s + 1) % 2,
                ));
            }
        }
        seen[seen.len() - 1][seen[0].len() - 1][0].min(seen[seen.len() - 1][seen[0].len() - 1][1])
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_3342() {
        assert_eq!(Solution::min_time_to_reach(vec![vec![0, 4], vec![4, 4]]), 7);
        assert_eq!(
            Solution::min_time_to_reach(vec![vec![0, 0, 0, 0], vec![0, 0, 0, 0]]),
            6
        );
        assert_eq!(Solution::min_time_to_reach(vec![vec![0, 1], vec![1, 2]]), 4);
    }
}
