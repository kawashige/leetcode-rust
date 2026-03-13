pub struct Solution {}

impl Solution {
    pub fn min_xor(nums: Vec<i32>, k: i32) -> i32 {
        let mut dp = vec![vec![std::i32::MAX; k as usize + 1]; nums.len()];
        let mut acc = vec![0; nums.len()];
        dp[0][1] = nums[0];
        acc[0] = nums[0];
        for i in 1..nums.len() {
            acc[i] = acc[i - 1] ^ nums[i];
            dp[i][1] = acc[i];
            for j in 0..i {
                for l in 1..k as usize {
                    if dp[j][l] != std::i32::MAX {
                        dp[i][l + 1] = dp[i][l + 1].min(dp[j][l].max(acc[i] ^ acc[j]))
                    }
                }
            }
        }

        dp[nums.len() - 1][k as usize]
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_3599() {
        assert_eq!(Solution::min_xor(vec![1, 2, 3], 2), 1);
        assert_eq!(Solution::min_xor(vec![2, 3, 3, 2], 3), 2);
        assert_eq!(Solution::min_xor(vec![1, 1, 2, 3, 1], 2), 0);
    }
}
