use std::collections::VecDeque;

pub struct Solution {}

impl Solution {
    pub fn is_ok(mid: i32, factors: &Vec<Vec<i32>>) -> bool {
        let mut stack = vec![(0, 0)];
        let mut seen = vec![vec![false; factors[0].len()]; factors.len()];

        while let Some((i, j)) = stack.pop() {
            if seen[i][j] || factors[i][j] < mid {
                continue;
            }
            if i == factors.len() - 1 && j == factors[0].len() - 1 {
                return true;
            }
            seen[i][j] = true;
            for (di, dj) in [(-1, 0), (0, -1), (0, 1), (1, 0)].iter() {
                let (new_i, new_j) = (i as i32 + di, j as i32 + dj);
                if !(0..factors.len() as i32).contains(&new_i)
                    || !(0..factors[0].len() as i32).contains(&new_j)
                    || factors[new_i as usize][new_j as usize] < mid
                {
                    continue;
                }
                stack.push((new_i as usize, new_j as usize));
            }
        }

        false
    }

    pub fn maximum_safeness_factor(grid: Vec<Vec<i32>>) -> i32 {
        let mut factors = vec![vec![std::i32::MAX; grid[0].len()]; grid.len()];
        let mut queue = VecDeque::new();
        for i in 0..grid.len() {
            for j in 0..grid[0].len() {
                if grid[i][j] == 1 {
                    queue.push_back(((i, j), 0));
                }
            }
        }
        while let Some(((i, j), factor)) = queue.pop_front() {
            if factors[i][j] <= factor {
                continue;
            }
            factors[i][j] = factor;
            for (di, dj) in [(-1, 0), (0, -1), (0, 1), (1, 0)].iter() {
                let (new_i, new_j) = (i as i32 + di, j as i32 + dj);
                if !(0..grid.len() as i32).contains(&new_i)
                    || !(0..grid[0].len() as i32).contains(&new_j)
                    || factors[new_i as usize][new_j as usize] <= factor + 1
                {
                    continue;
                }
                queue.push_back(((new_i as usize, new_j as usize), factor + 1));
            }
        }

        let mut ok = 0;
        let mut ng = (grid.len() + grid[0].len()) as i32;

        while ok + 1 < ng {
            let mid = (ok + ng) / 2;
            if Self::is_ok(mid, &factors) {
                ok = mid;
            } else {
                ng = mid;
            }
        }

        ok
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_2812() {
        assert_eq!(
            Solution::maximum_safeness_factor(vec![vec![1, 0, 0], vec![0, 0, 0], vec![0, 0, 1]]),
            0
        );
        assert_eq!(
            Solution::maximum_safeness_factor(vec![vec![0, 0, 1], vec![0, 0, 0], vec![0, 0, 0]]),
            2
        );
        assert_eq!(
            Solution::maximum_safeness_factor(vec![
                vec![0, 0, 0, 1],
                vec![0, 0, 0, 0],
                vec![0, 0, 0, 0],
                vec![1, 0, 0, 0]
            ]),
            2
        );
    }
}
