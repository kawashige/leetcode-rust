use std::collections::BinaryHeap;

pub struct Solution {}

impl Solution {
    pub fn constrained_subset_sum(nums: Vec<i32>, k: i32) -> i32 {
        let mut max_sum = std::i32::MIN;
        let mut heap = BinaryHeap::new();

        for i in 0..nums.len() {
            while let Some((sum, j)) = heap.pop() {
                if i <= j + k as usize {
                    heap.push((sum, j));
                    heap.push((sum + nums[i], i));
                    max_sum = max_sum.max(sum + nums[i]);
                    break;
                }
            }
            heap.push((nums[i], i));
            max_sum = max_sum.max(nums[i]);
        }

        max_sum
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_1425() {
        assert_eq!(
            Solution::constrained_subset_sum(vec![10, 2, -10, 5, 20], 2),
            37
        );
        assert_eq!(Solution::constrained_subset_sum(vec![-1, -2, -3], 1), -1);
        assert_eq!(
            Solution::constrained_subset_sum(vec![10, -2, -10, -5, 20], 2),
            23
        );
    }
}
