use std::{cmp::Reverse, collections::BinaryHeap};

pub struct Solution {}

impl Solution {
    pub fn max_score(nums1: Vec<i32>, nums2: Vec<i32>, k: i32) -> i64 {
        let k = k as usize;
        let mut nums = nums2.into_iter().zip(nums1.into_iter()).collect::<Vec<_>>();
        nums.sort_unstable_by(|a, b| b.0.cmp(&a.0));
        let mut heap = BinaryHeap::new();

        let mut sum = 0;

        for i in 0..k {
            sum += nums[i].1 as i64;
            heap.push(Reverse(nums[i].1 as i64));
        }

        let mut result = sum * nums[k - 1].0 as i64;

        for i in k..nums.len() {
            sum += nums[i].1 as i64;
            heap.push(Reverse(nums[i].1 as i64));
            if let Some(Reverse(num)) = heap.pop() {
                sum -= num;
            }
            result = result.max(sum * nums[i].0 as i64);
        }

        result
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_2542() {
        assert_eq!(
            Solution::max_score(vec![1, 3, 3, 2], vec![2, 1, 3, 4], 3),
            12
        );
        assert_eq!(
            Solution::max_score(vec![4, 2, 3, 1, 1], vec![7, 5, 10, 9, 6], 1),
            30
        );
    }
}
