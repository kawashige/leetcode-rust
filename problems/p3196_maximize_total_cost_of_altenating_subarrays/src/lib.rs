pub struct Solution {}

impl Solution {
    pub fn maximum_total_cost(nums: Vec<i32>) -> i64 {
        let mut dp = vec![0; nums.len() + 1];
        for i in 0..nums.len() {
            if 0 < i && nums[i] < 0 {
                dp[i + 1] =
                    (dp[i - 1] + nums[i - 1] as i64 - nums[i] as i64).max(dp[i] + nums[i] as i64);
            } else {
                dp[i + 1] = dp[i] + nums[i] as i64;
            }
        }
        *dp.last().unwrap()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_3196() {
        assert_eq!(Solution::maximum_total_cost(vec![1, -2, 3, 4]), 10);
        assert_eq!(Solution::maximum_total_cost(vec![1, -1, 1, -1]), 4);
        assert_eq!(Solution::maximum_total_cost(vec![0]), 0);
        assert_eq!(Solution::maximum_total_cost(vec![1, -1]), 2);
    }
}
