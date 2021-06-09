use std::collections::BinaryHeap;

pub struct Solution {}

impl Solution {
    pub fn max_result(nums: Vec<i32>, k: i32) -> i32 {
        let k = k as usize;
        let mut heap = BinaryHeap::new();
        heap.push((nums[0], 0));

        let mut r = nums[0];
        for i in 1..nums.len() {
            let mut max = 0;
            while let Some((x, j)) = heap.peek() {
                if i <= j + k {
                    max = *x;
                    break;
                }
                heap.pop();
            }
            r = nums[i] + max;
            heap.push((r, i));
        }

        r
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_day09() {
        assert_eq!(Solution::max_result(vec![-123], 10), -123);
        assert_eq!(Solution::max_result(vec![1, -1, -2, 4, -7, 3], 2), 7);
        assert_eq!(Solution::max_result(vec![10, -5, -2, 4, 0, 3], 3), 17);
        assert_eq!(
            Solution::max_result(vec![1, -5, -20, 4, -1, 3, -6, -3], 2),
            0
        );
    }
}
