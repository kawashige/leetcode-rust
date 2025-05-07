pub struct Solution {}

impl Solution {
    pub fn min_time_to_reach(move_time: Vec<Vec<i32>>) -> i32 {
        let mut seen = vec![vec![std::i32::MAX; move_time[0].len()]; move_time.len()];
        let mut stack = vec![(0, (0, 0))];
        while let Some((t, (i, j))) = stack.pop() {
            if seen[i][j] <= t {
                continue;
            }
            seen[i][j] = t;
            for (di, dj) in [(-1, 0), (0, -1), (0, 1), (1, 0)].iter() {
                let (new_i, new_j) = (i as i32 + di, j as i32 + dj);
                if !(0..move_time.len() as i32).contains(&new_i)
                    || !(0..move_time[0].len() as i32).contains(&new_j)
                {
                    continue;
                }
                let new_t = t.max(move_time[new_i as usize][new_j as usize]) + 1;
                stack.push((new_t, (new_i as usize, new_j as usize)));
            }
        }
        seen[seen.len() - 1][seen[0].len() - 1]
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_3341() {
        assert_eq!(Solution::min_time_to_reach(vec![vec![0, 4], vec![4, 4]]), 6);
        assert_eq!(
            Solution::min_time_to_reach(vec![vec![0, 0, 0], vec![0, 0, 0]]),
            3
        );
        assert_eq!(Solution::min_time_to_reach(vec![vec![0, 1], vec![1, 2]]), 3);
    }
}
