use std::{cmp::Reverse, collections::BinaryHeap};

pub struct Solution {}

impl Solution {
    pub fn total_cost(costs: Vec<i32>, k: i32, candidates: i32) -> i64 {
        let candidates = candidates as usize;
        if costs.len() <= k as usize {
            return costs.into_iter().map(|cost| cost as i64).sum::<i64>();
        }

        let mut heap = BinaryHeap::new();
        for i in 0..candidates as usize {
            heap.push((Reverse(costs[i]), Reverse(i)));
        }
        for i in candidates.max(costs.len() - candidates)..costs.len() {
            heap.push((Reverse(costs[i]), Reverse(i)));
        }

        let mut result = 0;
        let mut left = candidates - 1;
        let mut right = costs.len() - candidates;

        for _ in 0..k {
            if let Some((Reverse(cost), Reverse(i))) = heap.pop() {
                result += cost as i64;
                if left + 1 < right {
                    if i <= left {
                        left += 1;
                        heap.push((Reverse(costs[left]), Reverse(left)));
                    } else {
                        right -= 1;
                        heap.push((Reverse(costs[right]), Reverse(right)));
                    }
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
    fn test_2462() {
        assert_eq!(
            Solution::total_cost(vec![17, 12, 10, 2, 7, 2, 11, 20, 8], 3, 4),
            11
        );
        assert_eq!(Solution::total_cost(vec![1, 2, 4, 1], 3, 3), 4);
    }
}
