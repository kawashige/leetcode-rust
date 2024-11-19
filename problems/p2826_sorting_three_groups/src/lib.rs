pub struct Solution {}

impl Solution {
    pub fn minimum_operations(nums: Vec<i32>) -> i32 {
        let mut dp = vec![vec![0; 3]; nums.len() + 1];
        for i in 0..nums.len() {
            match nums[i] {
                1 => {
                    dp[i + 1][0] = dp[i][0];
                    dp[i + 1][1] = dp[i][1] + 1;
                    dp[i + 1][2] = dp[i][2] + 1;
                }
                2 => {
                    dp[i + 1][0] = dp[i][0] + 1;
                    dp[i + 1][1] = dp[i][0].min(dp[i][1]);
                    dp[i + 1][2] = dp[i][2] + 1;
                }
                3 => {
                    dp[i + 1][0] = dp[i][0] + 1;
                    dp[i + 1][1] = dp[i][1] + 1;
                    dp[i + 1][2] = dp[i][0].min(dp[i][1]).min(dp[i][2]);
                }
                _ => unreachable!(),
            }
        }
        *dp.last().unwrap().into_iter().min().unwrap()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_2826() {
        assert_eq!(Solution::minimum_operations(vec![2, 3, 2, 2]), 1);
        assert_eq!(Solution::minimum_operations(vec![2, 1, 3, 2, 1]), 3);
        assert_eq!(Solution::minimum_operations(vec![1, 3, 2, 1, 3, 3]), 2);
        assert_eq!(Solution::minimum_operations(vec![2, 2, 2, 2, 3, 3]), 0);
    }
}
