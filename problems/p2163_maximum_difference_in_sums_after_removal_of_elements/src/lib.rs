use std::{cmp::Reverse, collections::BinaryHeap};

pub struct Solution {}

impl Solution {
    pub fn minimum_difference(nums: Vec<i32>) -> i64 {
        let mut first_heap = BinaryHeap::new();
        let mut first_sum = 0;
        let mut first_sums = vec![0; nums.len()];
        for i in 0..nums.len() / 3 {
            first_heap.push(nums[i]);
            first_sum += nums[i] as i64;
        }
        first_sums[nums.len() / 3 - 1] = first_sum;
        for i in nums.len() / 3..(nums.len() * 2) / 3 {
            first_heap.push(nums[i]);
            first_sum += nums[i] as i64;
            first_sum -= first_heap.pop().unwrap() as i64;
            first_sums[i] = first_sum;
        }

        let mut second_heap = BinaryHeap::new();
        let mut second_sum = 0;
        for i in (nums.len() * 2) / 3..nums.len() {
            second_heap.push(Reverse(nums[i]));
            second_sum += nums[i] as i64;
        }
        let mut result = first_sums[(nums.len() * 2) / 3 - 1] - second_sum;
        for i in (nums.len() / 3..(nums.len() * 2) / 3).rev() {
            second_heap.push(Reverse(nums[i]));
            second_sum += nums[i] as i64;
            if let Some(Reverse(x)) = second_heap.pop() {
                second_sum -= x as i64;
            }
            result = result.min(first_sums[i - 1] - second_sum);
        }

        result
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_2163() {
        assert_eq!(Solution::minimum_difference(vec![3, 1, 2]), -1);
        assert_eq!(Solution::minimum_difference(vec![7, 9, 5, 8, 1, 3]), 1);
    }
}
