pub struct Solution {}

impl Solution {
    pub fn rob(nums: Vec<i32>, colors: Vec<i32>) -> i64 {
        let mut dp = vec![0; nums.len() + 1];
        dp[1] = nums[0] as i64;

        for i in 1..nums.len() {
            if colors[i - 1] == colors[i] {
                if 2 < i && colors[i - 3] == colors[i - 2] {
                    dp[i + 1] = dp[i - 2].max(dp[i - 1]) + nums[i] as i64;
                } else {
                    dp[i + 1] = dp[i - 1] + nums[i] as i64;
                }
            } else {
                if 1 < i && colors[i - 2] == colors[i - 1] {
                    dp[i + 1] = dp[i - 1].max(dp[i]) + nums[i] as i64;
                } else {
                    dp[i + 1] = dp[i] + nums[i] as i64
                }
            }
        }

        dp[dp.len() - 1].max(dp[dp.len() - 2])
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_3840() {
        assert_eq!(Solution::rob(vec![1, 4, 3, 5], vec![1, 1, 2, 2]), 9);
        assert_eq!(Solution::rob(vec![3, 1, 2, 4], vec![2, 3, 2, 2]), 8);
        assert_eq!(Solution::rob(vec![10, 1, 3, 9], vec![1, 1, 1, 2]), 22);
    }
}

fn main() {}
