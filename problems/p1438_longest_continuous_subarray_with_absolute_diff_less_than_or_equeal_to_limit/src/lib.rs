use std::collections::BinaryHeap;

pub struct Solution {}

impl Solution {
    pub fn longest_subarray(nums: Vec<i32>, limit: i32) -> i32 {
        let mut max_heap = BinaryHeap::new();
        let mut min_heap = BinaryHeap::new();
        let mut l = 0;
        let mut result = 0;

        for r in 0..nums.len() {
            max_heap.push((nums[r], r));
            min_heap.push((-nums[r], r));

            while limit < max_heap.peek().unwrap().0 + min_heap.peek().unwrap().0 {
                l += 1;
                while let Some((num, i)) = max_heap.pop() {
                    if l <= i {
                        max_heap.push((num, i));
                        break;
                    }
                }
                while let Some((num, i)) = min_heap.pop() {
                    if l <= i {
                        min_heap.push((num, i));
                        break;
                    }
                }
            }

            result = result.max(r - l + 1);
        }

        result as i32
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_1438() {
        assert_eq!(Solution::longest_subarray(vec![8, 2, 4, 7], 4), 2);
        assert_eq!(Solution::longest_subarray(vec![10, 1, 2, 4, 7, 2], 5), 4);
        assert_eq!(
            Solution::longest_subarray(vec![4, 2, 2, 2, 4, 4, 2, 2,], 0),
            3
        );
    }
}
