pub struct Solution {}

impl Solution {
    pub fn length_of_longest_subsequence(nums: Vec<i32>, target: i32) -> i32 {
        let target = target as usize;
        let mut dp = vec![vec![-1; target + 1]; nums.len() + 1];
        dp[0][0] = 0;

        for i in 0..nums.len() {
            for j in 0..dp[i].len() {
                if dp[i][j] != -1 {
                    dp[i + 1][j] = dp[i][j].max(dp[i + 1][j]);
                    if (j + nums[i] as usize) < dp[i].len() {
                        dp[i + 1][j + nums[i] as usize] =
                            dp[i + 1][j + nums[i] as usize].max(dp[i][j] + 1);
                    }
                }
            }
        }

        dp[dp.len() - 1][target]
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_2915() {
        assert_eq!(
            Solution::length_of_longest_subsequence(vec![1, 2, 3, 4, 5], 9),
            3
        );
        assert_eq!(
            Solution::length_of_longest_subsequence(vec![4, 1, 3, 2, 1, 5], 7),
            4
        );
        assert_eq!(
            Solution::length_of_longest_subsequence(vec![1, 1, 5, 4, 5], 3),
            -1
        );
    }
}
