pub struct Solution {}

impl Solution {
    pub fn maximum_difference(nums: Vec<i32>) -> i32 {
        let mut max = nums[nums.len() - 1];
        let mut max_diff = -1;

        for i in (0..nums.len() - 1).rev() {
            if nums[i] < max {
                max_diff = max_diff.max(max - nums[i]);
            }
            max = max.max(nums[i]);
        }

        max_diff
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_2016() {
        assert_eq!(Solution::maximum_difference(vec![7, 1, 5, 4]), 4);
        assert_eq!(Solution::maximum_difference(vec![9, 4, 3, 2]), -1);
        assert_eq!(Solution::maximum_difference(vec![1, 5, 2, 10]), 9);
    }
}
