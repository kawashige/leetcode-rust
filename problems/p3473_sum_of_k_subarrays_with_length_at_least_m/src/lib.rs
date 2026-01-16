pub struct Solution {}

impl Solution {
    pub fn max_sum(nums: Vec<i32>, k: i32, m: i32) -> i32 {
        let m = m as usize;
        let mut dp = vec![vec![-100_000_000; nums.len() + 1]; k as usize + 1];
        dp[0][0] = 0;
        let mut prefix_sum = vec![0; nums.len() + 1];
        for i in 0..nums.len() {
            prefix_sum[i + 1] = prefix_sum[i] + nums[i];
        }

        for i in 0..nums.len() {
            dp[0][i] = 0;
        }

        for i in 0..k as usize {
            let mut best = -100_000_000;
            for j in 0..=nums.len() {
                if 0 < j {
                    dp[i + 1][j] = dp[i + 1][j].max(dp[i + 1][j - 1]);
                }
                if m <= j {
                    best = best.max(dp[i][j - m] - prefix_sum[j - m]);
                }
                if best != -100_000_000 {
                    dp[i + 1][j] = dp[i + 1][j].max(prefix_sum[j] + best);
                }
            }
        }

        dp[k as usize][nums.len()]
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_3473() {
        assert_eq!(Solution::max_sum(vec![1, 2, -1, 3, 3, 4], 2, 2), 13);
        assert_eq!(Solution::max_sum(vec![-10, 3, -1, -2], 4, 1), -10);
    }
}
