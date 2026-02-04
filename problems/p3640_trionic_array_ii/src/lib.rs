pub struct Solution {}

impl Solution {
    pub fn max_sum_trionic(nums: Vec<i32>) -> i64 {
        let mut dp = vec![vec![std::i64::MIN; nums.len()]; 4];
        dp[0][0] = nums[0] as i64;

        for i in 1..nums.len() {
            dp[0][i] = nums[i] as i64;
            if nums[i - 1] < nums[i] {
                dp[0][i] = dp[0][i].max(dp[0][i - 1] + nums[i] as i64);
                if dp[1][i - 1] != std::i64::MIN {
                    dp[1][i] = dp[1][i].max(dp[1][i - 1] + nums[i] as i64);
                }
                dp[1][i] = dp[1][i].max(dp[0][i - 1] + nums[i] as i64);
                if dp[3][i - 1] != std::i64::MIN {
                    dp[3][i] = dp[3][i].max(dp[3][i - 1] + nums[i] as i64);
                }
                if dp[2][i - 1] != std::i64::MIN {
                    dp[3][i] = dp[3][i].max(dp[2][i - 1] + nums[i] as i64);
                }
            } else if nums[i - 1] > nums[i] {
                if dp[2][i - 1] != std::i64::MIN {
                    dp[2][i] = dp[2][i].max(dp[2][i - 1] + nums[i] as i64);
                }
                if dp[1][i - 1] != std::i64::MIN {
                    dp[2][i] = dp[2][i].max(dp[1][i - 1] + nums[i] as i64);
                }
            }
        }

        *dp[3].iter().max().unwrap()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_3640() {
        assert_eq!(Solution::max_sum_trionic(vec![0, -2, -1, -3, 0, 2, -1]), -4);
        assert_eq!(Solution::max_sum_trionic(vec![1, 4, 2, 7]), 14);
    }
}
