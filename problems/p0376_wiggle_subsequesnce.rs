pub struct Solution {}

impl Solution {
    pub fn wiggle_max_length(nums: Vec<i32>) -> i32 {
        if nums.len() < 2 {
            return nums.len() as i32;
        }
        let mut dp = vec![1; nums.len()];
        let mut last_minus = 0;
        let mut last_plus = 0;
        for i in 1..nums.len() {
            if nums[i - 1] < nums[i] {
                last_plus = i;
                dp[i] = dp[last_minus] + 1;
            } else if nums[i - 1] > nums[i] {
                last_minus = i;
                dp[i] = dp[last_plus] + 1;
            }
        }
        std::cmp::max(dp[last_minus], dp[last_plus])
    }
}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn test_0376() {
        assert_eq!(0, Solution::wiggle_max_length(vec![]));
        assert_eq!(1, Solution::wiggle_max_length(vec![1]));
        assert_eq!(2, Solution::wiggle_max_length(vec![1, 2]));
        assert_eq!(1, Solution::wiggle_max_length(vec![0, 0]));
        assert_eq!(6, Solution::wiggle_max_length(vec![1, 7, 4, 9, 2, 5]));
        assert_eq!(
            7,
            Solution::wiggle_max_length(vec![1, 17, 5, 10, 13, 15, 10, 5, 16, 8])
        );
        assert_eq!(
            2,
            Solution::wiggle_max_length(vec![1, 2, 3, 4, 5, 6, 7, 8, 9])
        );
    }
}
