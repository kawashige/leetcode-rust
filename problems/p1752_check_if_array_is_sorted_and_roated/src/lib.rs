pub struct Solution {}

impl Solution {
    pub fn check(nums: Vec<i32>) -> bool {
        let min = nums.iter().min().unwrap();
        let mut i = (0..nums.len()).rev().find(|i| &nums[*i] == min).unwrap();
        while 0 < i && nums[i - 1] == nums[i] {
            i -= 1;
        }
        (1..nums.len()).all(|j| nums[(i + j - 1) % nums.len()] <= nums[(i + j) % nums.len()])
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_1752() {
        assert!(Solution::check(vec![3, 4, 5, 1, 2]));
        assert!(!Solution::check(vec![2, 1, 3, 4]));
        assert!(Solution::check(vec![1, 2, 3]));
        assert!(Solution::check(vec![1, 1, 1]));
        assert!(Solution::check(vec![7, 9, 1, 1, 1, 3]));
    }
}
