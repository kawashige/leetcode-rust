pub struct Solution {}

impl Solution {
    pub fn minimize_sum(nums: Vec<i32>) -> i32 {
        let mut nums = nums;
        nums.sort_unstable();

        (nums[nums.len() - 1] - nums[2])
            .min(nums[nums.len() - 3] - nums[0])
            .min(nums[nums.len() - 2] - nums[1])
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_2567() {
        assert_eq!(Solution::minimize_sum(vec![1, 4, 7, 8, 5]), 3);
        assert_eq!(Solution::minimize_sum(vec![1, 4, 3]), 0);
    }
}
