pub struct Solution {}

impl Solution {
    pub fn count_good_strings(low: i32, high: i32, zero: i32, one: i32) -> i32 {
        const M: usize = 1_000_000_007;
        let zero = zero as usize;
        let one = one as usize;

        let mut dp = vec![0; high as usize + 1];
        dp[0] = 1;
        for i in 1..dp.len() {
            if zero <= i {
                dp[i] += dp[i - zero];
                dp[i] %= M;
            }
            if one <= i {
                dp[i] += dp[i - one];
                dp[i] %= M;
            }
        }

        (low as usize..=high as usize).fold(0, |acc, i| (acc + dp[i]) % M) as i32
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_2466() {
        assert_eq!(Solution::count_good_strings(3, 3, 1, 1), 8);
        assert_eq!(Solution::count_good_strings(2, 3, 1, 2), 5);
    }
}
