use std::collections::VecDeque;

pub struct Solution {}

impl Solution {
    pub fn check_array(nums: Vec<i32>, k: i32) -> bool {
        let mut d = 0;
        let k = k as usize;
        let mut deque = VecDeque::new();
        for i in 0..nums.len() {
            if nums[i] < d {
                return false;
            }
            deque.push_back(nums[i] - d);
            d += nums[i] - d;
            if k - 1 < deque.len() {
                d -= deque.pop_front().unwrap();
            }
        }
        d == 0
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_2772() {
        assert!(Solution::check_array(vec![2, 2, 3, 1, 1, 0], 3));
        assert!(!Solution::check_array(vec![1, 3, 1, 1], 2));
    }
}
