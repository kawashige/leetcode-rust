pub struct Solution {}

impl Solution {
    pub fn maximum_length(nums: Vec<i32>, k: i32) -> i32 {
        let mut dp = vec![vec![1; k as usize]; nums.len()];
        let mut result = std::i32::MIN;

        for i in 0..nums.len() {
            for j in 0..i {
                dp[i][((nums[i] + nums[j]) % k) as usize] = dp[i]
                    [((nums[i] + nums[j]) % k) as usize]
                    .max(dp[j][((nums[i] + nums[j]) % k) as usize] + 1);
                result = result.max(dp[i][((nums[i] + nums[j]) % k) as usize]);
            }
        }

        result
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_3202() {
        assert_eq!(Solution::maximum_length(vec![1, 2, 3, 4, 5], 2), 5);
        assert_eq!(Solution::maximum_length(vec![1, 4, 2, 3, 1, 4], 3), 4);
    }
}
