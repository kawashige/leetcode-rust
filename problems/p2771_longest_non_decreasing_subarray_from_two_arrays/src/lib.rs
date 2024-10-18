pub struct Solution {}

impl Solution {
    pub fn max_non_decreasing_length(nums1: Vec<i32>, nums2: Vec<i32>) -> i32 {
        let mut dp = vec![vec![1; 2]; nums1.len()];
        dp[0][0] = 1;
        dp[0][1] = 1;
        let nums = vec![nums1, nums2];
        let mut result = 1;

        for i in 1..dp.len() {
            for j in 0..nums.len() {
                for k in 0..nums.len() {
                    if nums[k][i - 1] <= nums[j][i] {
                        dp[i][j] = dp[i][j].max(dp[i - 1][k] + 1);
                        result = result.max(dp[i][j]);
                    }
                }
            }
        }

        result
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_2771() {
        assert_eq!(
            Solution::max_non_decreasing_length(vec![2, 3, 1], vec![1, 2, 1]),
            2
        );
        assert_eq!(
            Solution::max_non_decreasing_length(vec![1, 3, 2, 1], vec![2, 2, 3, 4]),
            4
        );
        assert_eq!(
            Solution::max_non_decreasing_length(vec![1, 1], vec![2, 2]),
            2
        );
    }
}
