pub struct Solution {}

impl Solution {
    pub fn rob(nums: Vec<i32>) -> i32 {
        let mut dp = Vec::with_capacity(nums.len());
        for i in 0..nums.len() {
            dp.push(if i == 0 {
                nums[i]
            } else if i == 1 {
                std::cmp::max(dp[i - 1], nums[i])
            } else {
                std::cmp::max(dp[i - 1], dp[i - 2] + nums[i])
            });
        }
        *dp.last().unwrap_or(&0)
    }
}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn test_day14() {
        assert_eq!(4, Solution::rob(vec![2, 1, 1, 2]));
        assert_eq!(4, Solution::rob(vec![1, 2, 3, 1]));
        assert_eq!(12, Solution::rob(vec![2, 7, 9, 3, 1]));
        assert_eq!(0, Solution::rob(vec![]));
    }
}
