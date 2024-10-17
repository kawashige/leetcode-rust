pub struct Solution {}

impl Solution {
    pub fn maximum_jumps(nums: Vec<i32>, target: i32) -> i32 {
        let mut dp = vec![-1; nums.len()];
        dp[0] = 0;
        for i in 1..nums.len() {
            for j in 0..i {
                if target < (nums[i] - nums[j]).abs() || dp[j] == -1 {
                    continue;
                }
                dp[i] = dp[i].max(dp[j] + 1);
            }
        }
        println!("{:?}", dp);
        dp[dp.len() - 1]
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_2770() {
        assert_eq!(Solution::maximum_jumps(vec![1, 3, 6, 4, 1, 2], 2), 3);
        assert_eq!(Solution::maximum_jumps(vec![1, 3, 6, 4, 1, 2], 3), 5);
        assert_eq!(Solution::maximum_jumps(vec![1, 3, 6, 4, 1, 2], 0), -1);
    }
}
