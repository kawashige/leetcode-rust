pub struct Solution {}

impl Solution {
    pub fn min_increment_operations(nums: Vec<i32>, k: i32) -> i64 {
        let mut dp = vec![std::i64::MAX; nums.len()];
        for i in 0..3 {
            dp[i] = (k - nums[i]).max(0) as i64;
        }

        for i in 3..nums.len() {
            for j in i - 3..i {
                dp[i] = dp[i].min((k - nums[i]).max(0) as i64 + dp[j]);
            }
        }
        dp.into_iter().rev().take(3).min().unwrap()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_2919() {
        assert_eq!(
            Solution::min_increment_operations(vec![13, 34, 0, 13, 9, 19], 82),
            117
        );
        assert_eq!(Solution::min_increment_operations(vec![1, 2, 6, 9], 8), 2);
        assert_eq!(
            Solution::min_increment_operations(vec![2, 3, 0, 0, 2], 4),
            3
        );
        assert_eq!(Solution::min_increment_operations(vec![0, 1, 3, 3], 5), 2);
        assert_eq!(Solution::min_increment_operations(vec![1, 1, 2], 1), 0);
    }
}
