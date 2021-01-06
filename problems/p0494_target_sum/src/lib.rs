pub struct Solution {}

impl Solution {
    pub fn find_target_sum_ways(nums: Vec<i32>, s: i32) -> i32 {
        if s < -1000 || 1000 < s || nums.is_empty() {
            return 0;
        }

        let mut dp = vec![vec![0; 2001]; nums.len()];
        dp[0][nums[0] as usize + 1000] += 1;
        dp[0][1000 - nums[0] as usize] += 1;
        for i in 1..nums.len() {
            for j in 0..2001 {
                if 0 < dp[i - 1][j] {
                    dp[i][j + nums[i] as usize] += dp[i - 1][j];
                    dp[i][j - nums[i] as usize] += dp[i - 1][j];
                }
            }
        }

        dp[nums.len() - 1][(s + 1000) as usize]
    }
}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn test_0494() {
        assert_eq!(
            256,
            Solution::find_target_sum_ways(vec![0, 0, 0, 0, 0, 0, 0, 0, 1], 1)
        );
        assert_eq!(5, Solution::find_target_sum_ways(vec![1, 1, 1, 1, 1], 3));
        assert_eq!(0, Solution::find_target_sum_ways(vec![], 3));
        assert_eq!(1, Solution::find_target_sum_ways(vec![1, 2, 3], -6));
    }
}
