pub struct Solution {}

impl Solution {
    pub fn find_min(nums: Vec<i32>) -> i32 {
        for i in 0..nums.len() {
            if nums[(nums.len() + i - 1) % nums.len()] > nums[i] {
                return nums[i];
            }
        }
        nums[0]
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_0154() {
        assert_eq!(Solution::find_min(vec![1, 3, 5]), 1);
        assert_eq!(Solution::find_min(vec![2, 2, 0, 1, 0]), 0);
        assert_eq!(Solution::find_min(vec![2, 2, 2]), 2);
    }
}
