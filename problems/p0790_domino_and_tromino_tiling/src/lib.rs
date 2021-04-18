pub struct Solution {}

impl Solution {
    pub fn num_tilings(n: i32) -> i32 {
        let n = n as usize;
        let m = 1_000_000_007;

        let mut dp = vec![0; std::cmp::max(n + 1, 4)];
        dp[1] = 1;
        dp[2] = 2;
        dp[3] = 5;

        for i in 4..=n {
            dp[i] = dp[i - 1] * 2 % m + dp[i - 3];
            dp[i] %= m;
        }

        dp[n]
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_0790() {
        assert_eq!(Solution::num_tilings(30), 312342182);
        assert_eq!(Solution::num_tilings(5), 24);
        assert_eq!(Solution::num_tilings(2), 2);
        assert_eq!(Solution::num_tilings(3), 5);
    }
}
