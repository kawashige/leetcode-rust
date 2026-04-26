pub struct Solution {}

impl Solution {
    pub fn max_product(nums: Vec<i32>) -> i64 {
        let bit_width = (32 - nums.iter().max().unwrap().leading_zeros()) as usize;
        let mut dp = vec![0; 1 << bit_width];

        for i in 0..nums.len() {
            dp[nums[i] as usize] = nums[i] as i64;
        }

        for i in 0..bit_width {
            for j in 0..(1 << bit_width) {
                if j & 1 << i != 0 {
                    dp[j] = dp[j].max(dp[j ^ 1 << i]);
                }
            }
        }

        let mut result = 0;
        let mask = (1 << bit_width) - 1;
        for i in 0..nums.len() {
            result = result.max(nums[i] as i64 * dp[(!nums[i] & mask) as usize]);
        }

        result
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_3670() {
        assert_eq!(Solution::max_product(vec![1, 2, 3, 4, 5, 6, 7]), 12);
        assert_eq!(Solution::max_product(vec![5, 6, 4]), 0);
        assert_eq!(Solution::max_product(vec![64, 8, 32]), 2048);
    }
}
