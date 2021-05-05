pub struct Solution {}

impl Solution {
    pub fn jump(nums: Vec<i32>) -> i32 {
        let mut dp = vec![0; nums.len()];
        for i in 1..nums.len() {
            if let Some(j) = (0..i).find(|j| j + nums[*j] as usize >= i) {
                dp[i] = dp[j] + 1;
            };
        }
        dp[nums.len() - 1] as i32
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_day05() {
        assert_eq!(Solution::jump(vec![2, 3, 1, 1, 4]), 2);
        assert_eq!(Solution::jump(vec![2, 3, 0, 1, 4]), 2);
    }
}
