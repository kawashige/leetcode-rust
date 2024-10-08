use std::{cmp::Reverse, collections::BinaryHeap};

pub struct Solution {}

impl Solution {
    pub fn continuous_subarrays(nums: Vec<i32>) -> i64 {
        let mut max_heap: BinaryHeap<(i32, usize)> = BinaryHeap::new();
        let mut min_heap: BinaryHeap<(Reverse<i32>, usize)> = BinaryHeap::new();
        let mut l = 0;
        let mut result = 0;

        for r in 0..nums.len() {
            while let Some((max, i)) = max_heap.pop() {
                if (max - nums[r]).abs() <= 2 {
                    max_heap.push((max, i));
                    break;
                }
                l = l.max(i + 1);
            }
            while let Some((Reverse(min), i)) = min_heap.pop() {
                if (min - nums[r]).abs() <= 2 {
                    min_heap.push((Reverse(min), i));
                    break;
                }
                l = l.max(i + 1);
            }

            result += (r - l + 1) as i64;
            max_heap.push((nums[r], r));
            min_heap.push((Reverse(nums[r]), r));
        }

        result
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_2762() {
        assert_eq!(Solution::continuous_subarrays(vec![5, 4, 2, 4]), 8);
        assert_eq!(Solution::continuous_subarrays(vec![1, 2, 3]), 6);
    }
}
