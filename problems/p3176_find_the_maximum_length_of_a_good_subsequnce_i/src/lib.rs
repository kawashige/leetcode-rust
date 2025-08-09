pub struct Solution {}

impl Solution {
    pub fn maximum_length(nums: Vec<i32>, k: i32) -> i32 {
        let mut dp = vec![vec![0; k as usize + 1]; nums.len()];
        let mut result = 1;

        for i in 0..nums.len() {
            dp[i][0] = 1;
            for j in 0..i {
                let d = if nums[i] == nums[j] { 0 } else { 1 };
                for l in 0..=k - d {
                    dp[i][(l + d) as usize] = dp[i][(l + d) as usize].max(dp[j][l as usize] + 1);
                    result = result.max(dp[i][(l + d) as usize]);
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
    fn test_3176() {
        assert_eq!(Solution::maximum_length(vec![1, 2, 1, 1, 3], 2), 4);
        assert_eq!(Solution::maximum_length(vec![1, 2, 3, 4, 5, 1], 0), 2);
    }
}
