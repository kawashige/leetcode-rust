pub struct Solution {}

impl Solution {
    pub fn valid_partition(nums: Vec<i32>) -> bool {
        let mut dp = vec![false; nums.len() + 1];
        dp[0] = true;

        for i in 1..nums.len() {
            if dp[i - 1] && nums[i - 1] == nums[i] {
                dp[i + 1] = true;
            }
            if 1 < i
                && dp[i - 2]
                && ((nums[i - 2] == nums[i - 1] && nums[i - 1] == nums[i])
                    || (nums[i - 2] + 1 == nums[i - 1] && nums[i - 1] + 1 == nums[i]))
            {
                dp[i + 1] = true;
            }
        }

        dp[nums.len()]
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_2369() {
        assert!(Solution::valid_partition(vec![4, 4, 4, 5, 6]));
        assert!(!Solution::valid_partition(vec![1, 1, 1, 2]));
    }
}
