use std::{cmp::Reverse, collections::BinaryHeap};

pub struct Solution {}

impl Solution {
    pub fn is_array_special(nums: Vec<i32>, queries: Vec<Vec<i32>>) -> Vec<bool> {
        let mut result = vec![true; queries.len()];

        let mut queries = queries.into_iter().zip(0..).collect::<Vec<_>>();
        queries.sort_unstable_by(|a, b| a.0[0].cmp(&b.0[0]).then(a.0[1].cmp(&b.0[1])));

        let mut heap = BinaryHeap::new();

        let mut j = 0;
        for i in 0..nums.len() {
            if 0 < i && nums[i - 1] % 2 == nums[i] % 2 {
                while let Some((_, l)) = heap.pop() {
                    result[l] = false
                }
            } else {
                while let Some((Reverse(k), l)) = heap.pop() {
                    if i < k {
                        heap.push((Reverse(k), l));
                        break;
                    }
                }
            }
            while j < queries.len() && queries[j].0[0] <= i as i32 {
                if queries[j].0[0] != queries[j].0[1] {
                    heap.push((Reverse(queries[j].0[1] as usize), queries[j].1));
                }
                j += 1;
            }
        }

        result
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_3152() {
        assert_eq!(
            Solution::is_array_special(vec![3, 4, 1, 2, 6], vec![vec![1, 4]]),
            vec![false]
        );
        assert_eq!(
            Solution::is_array_special(vec![4, 3, 1, 6], vec![vec![0, 2], vec![2, 3]]),
            vec![false, true]
        );
    }
}
