use std::{cmp::Reverse, collections::BinaryHeap};

pub struct Solution {}

impl Solution {
    pub fn num_of_minutes(n: i32, head_id: i32, manager: Vec<i32>, inform_time: Vec<i32>) -> i32 {
        let mut subordinates = vec![vec![]; n as usize];
        for (p, m) in manager.into_iter().enumerate() {
            if m != -1 {
                subordinates[m as usize].push(p);
            }
        }

        let mut heap = BinaryHeap::new();
        heap.push((Reverse(0), head_id as usize));

        let mut time = vec![std::i32::MAX; n as usize];
        let mut result = 0;

        while let Some((Reverse(t), p)) = heap.pop() {
            if time[p] < t {
                continue;
            }
            time[p] = t;
            result = result.max(t);

            for s in &subordinates[p] {
                if t + inform_time[p] < time[*s] {
                    time[*s] = t + inform_time[p];
                    heap.push((Reverse(time[*s]), *s));
                }
            }
        }

        result
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_1376() {
        assert_eq!(Solution::num_of_minutes(1, 0, vec![-1], vec![0]), 0);
        assert_eq!(
            Solution::num_of_minutes(6, 2, vec![2, 2, -1, 2, 2, 2], vec![0, 0, 1, 0, 0, 0]),
            1
        );
    }
}
