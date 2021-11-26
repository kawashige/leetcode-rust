pub struct Solution {}

impl Solution {
    pub fn longest_arith_seq_length(nums: Vec<i32>) -> i32 {
        let mut dp = vec![vec![1; 1001]; nums.len()];
        let mut r = 0;

        for i in 1..nums.len() {
            for j in 0..i {
                let d = (nums[i] - nums[j] + 500) as usize;
                dp[i][d] = dp[j][d] + 1;
                r = r.max(dp[i][d])
            }
        }

        r
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_1027() {
        assert_eq!(Solution::longest_arith_seq_length(vec![3, 6, 9, 12]), 4);
        assert_eq!(Solution::longest_arith_seq_length(vec![9, 4, 7, 2, 10]), 3);
        assert_eq!(
            Solution::longest_arith_seq_length(vec![20, 1, 15, 3, 10, 5, 8]),
            4
        );
    }
}
