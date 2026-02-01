pub struct Solution {}

impl Solution {
    pub fn result_array(nums: Vec<i32>, k: i32) -> Vec<i64> {
        let mut dp = vec![vec![0; k as usize]; nums.len()];
        dp[0][(nums[0] % k) as usize] = 1;
        let mut result = vec![0; k as usize];
        result[(nums[0] % k) as usize] = 1;

        for i in 1..nums.len() {
            dp[i][(nums[i] % k) as usize] = 1;
            for j in 0..dp[i].len() {
                dp[i][(j * (nums[i] % k) as usize) % k as usize] += dp[i - 1][j];
            }
            for j in 0..dp[i].len() {
                result[j] += dp[i][j];
            }
        }

        result
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_3524() {
        assert_eq!(
            Solution::result_array(vec![1, 2, 3, 4, 5], 3),
            vec![9, 2, 4]
        );
        assert_eq!(
            Solution::result_array(vec![1, 2, 4, 8, 16, 32], 4),
            vec![18, 1, 2, 0]
        );
        assert_eq!(Solution::result_array(vec![1, 1, 2, 1, 1], 2), vec![9, 6]);
    }
}
