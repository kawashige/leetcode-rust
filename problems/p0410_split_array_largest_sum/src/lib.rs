pub struct Solution {}

impl Solution {
    pub fn split_array(nums: Vec<i32>, m: i32) -> i32 {
        let mut acc = vec![0; nums.len() + 1];
        for i in 0..nums.len() {
            acc[i + 1] += nums[i] + acc[i];
        }

        let mut dp = vec![vec![-1; m as usize + 1]; nums.len() + 1];
        dp[0][0] = 0;

        for i in 0..nums.len() {
            for j in 0..=i {
                for k in 0..m as usize {
                    if dp[j][k] >= 0 {
                        dp[i + 1][k + 1] = if dp[i + 1][k + 1] < 0 {
                            dp[j][k].max(acc[i + 1] - acc[j])
                        } else {
                            dp[i + 1][k + 1].min(dp[j][k].max(acc[i + 1] - acc[j]))
                        };
                    }
                }
            }
        }

        dp[nums.len()][m as usize]
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_0410() {
        assert_eq!(Solution::split_array(vec![7, 2, 5, 10, 8], 2), 18);
        assert_eq!(Solution::split_array(vec![1, 2, 3, 4, 5], 2), 9);
        assert_eq!(Solution::split_array(vec![1, 4, 4], 3), 4);
    }
}
