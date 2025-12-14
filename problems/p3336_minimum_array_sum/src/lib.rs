pub struct Solution {}

impl Solution {
    pub fn min_array_sum(nums: Vec<i32>, k: i32, op1: i32, op2: i32) -> i32 {
        let op1 = op1 as usize;
        let op2 = op2 as usize;
        let r = k;
        let mut dp = vec![vec![vec![std::i32::MAX; op2 + 1]; op1 + 1]; nums.len() + 1];
        dp[0][op1][op2] = 0;

        for i in 0..nums.len() {
            for j in 0..=op1 {
                for k in 0..=op2 {
                    if dp[i][j][k] == std::i32::MAX {
                        continue;
                    }
                    // only op1
                    if 0 < j {
                        dp[i + 1][j - 1][k] =
                            dp[i + 1][j - 1][k].min(dp[i][j][k] + (nums[i] + 1) / 2);
                    }

                    // only op2
                    if 0 < k && r <= nums[i] {
                        dp[i + 1][j][k - 1] = dp[i + 1][j][k - 1].min(dp[i][j][k] + nums[i] - r);
                    }

                    // both
                    if 0 < k && 0 < j {
                        // op1 -> op2
                        if r <= (nums[i] + 1) / 2 {
                            dp[i + 1][j - 1][k - 1] =
                                dp[i + 1][j - 1][k - 1].min(dp[i][j][k] + ((nums[i] + 1) / 2) - r);
                        }

                        // op2 -> op1
                        if r <= nums[i] {
                            dp[i + 1][j - 1][k - 1] =
                                dp[i + 1][j - 1][k - 1].min(dp[i][j][k] + (nums[i] - r + 1) / 2);
                        }
                    }

                    // skip
                    dp[i + 1][j][k] = dp[i + 1][j][k].min(dp[i][j][k] + nums[i]);
                }
            }
        }

        let mut result = std::i32::MAX;
        for i in 0..=op1 {
            for j in 0..=op2 {
                result = result.min(dp[nums.len()][i][j]);
            }
        }
        result
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_3336() {
        assert_eq!(Solution::min_array_sum(vec![2, 8, 3, 19, 3], 3, 1, 1), 23);
        assert_eq!(Solution::min_array_sum(vec![2, 4, 3], 3, 2, 1), 3);
    }
}
