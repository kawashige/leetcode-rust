pub struct Solution {}

impl Solution {
    pub fn k_inverse_pairs(n: i32, k: i32) -> i32 {
        let m = 1_000_000_007;
        let mut dp = vec![0; k as usize + 1];
        dp[0] = 1;

        for i in 1..(n as usize) {
            let mut new_dp = vec![0; k as usize + 1];

            let mut sum = 0;
            for j in 0..dp.len() {
                sum = (sum + dp[j]) % m;
                if j >= i + 1 {
                    sum -= dp[j - i - 1];
                    if sum < 0 {
                        sum += m;
                    }
                }
                new_dp[j] = sum % m;
            }

            dp = new_dp;
        }

        dp[k as usize]
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_day19() {
        assert_eq!(Solution::k_inverse_pairs(2, 2), 0);
        assert_eq!(Solution::k_inverse_pairs(3, 0), 1);
        assert_eq!(Solution::k_inverse_pairs(3, 1), 2);
    }
}
