pub struct Solution {}

impl Solution {
    pub fn total_steps(nums: Vec<i32>) -> i32 {
        let mut dp = vec![0; nums.len()];
        let mut stack = vec![nums.len() - 1];

        for i in (0..nums.len() - 1).rev() {
            while let Some(j) = stack.pop() {
                if nums[j] >= nums[i] {
                    stack.push(j);
                    break;
                }
                dp[i] = (dp[i] + 1).max(dp[j]);
            }
            stack.push(i);
        }

        dp.into_iter().max().unwrap()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_2289() {
        assert_eq!(Solution::total_steps(vec![5, 14, 15, 2, 11, 5, 13, 15]), 3);
        assert_eq!(
            Solution::total_steps(vec![10, 1, 2, 3, 4, 5, 6, 1, 2, 3]),
            6
        );
        assert_eq!(
            Solution::total_steps(vec![5, 3, 4, 4, 7, 3, 6, 11, 8, 5, 11]),
            3
        );
        assert_eq!(Solution::total_steps(vec![4, 5, 7, 7, 13]), 0);
    }
}
