use std::{cmp::Reverse, collections::BinaryHeap};

pub struct Solution {}

impl Solution {
    pub fn network_delay_time(times: Vec<Vec<i32>>, n: i32, k: i32) -> i32 {
        let n = n as usize;
        let k = k as usize;
        let mut lists = vec![vec![]; n + 1];

        for t in times {
            lists[t[0] as usize].push((t[1] as usize, t[2]));
        }

        let mut dist: Vec<i32> = vec![-1; n + 1];
        dist[k] = 0;
        let mut heap = BinaryHeap::new();
        heap.push(Reverse((0, k)));

        while let Some(Reverse((t, n))) = heap.pop() {
            if t > dist[n] {
                continue;
            }

            for (n2, w) in &lists[n] {
                if dist[*n2] == -1 || dist[*n2] > w + dist[n] {
                    dist[*n2] = w + dist[n];
                    heap.push(Reverse((dist[n] + w, *n2)));
                }
            }
        }

        let mut max = 0;
        for i in 1..=n {
            if dist[i] == -1 {
                return -1;
            }
            max = std::cmp::max(max, dist[i]);
        }
        max
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_0743() {
        assert_eq!(
            Solution::network_delay_time(vec![vec![2, 1, 1], vec![2, 3, 1], vec![3, 4, 1]], 4, 2),
            2
        );
        assert_eq!(Solution::network_delay_time(vec![vec![1, 2, 1]], 2, 1), 1);
        assert_eq!(Solution::network_delay_time(vec![vec![1, 2, 1]], 2, 2), -1);
    }
}
