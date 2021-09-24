pub struct Solution {}

impl Solution {
    pub fn tribonacci(n: i32) -> i32 {
        let mut dp = vec![0; 38];
        dp[1] = 1;
        dp[2] = 1;

        for i in 3..=(n as usize) {
            dp[i] = dp[i - 1] + dp[i - 2] + dp[i - 3];
        }

        dp[n as usize]
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_day24() {
        assert_eq!(Solution::tribonacci(0), 0);
        assert_eq!(Solution::tribonacci(4), 4);
        assert_eq!(Solution::tribonacci(25), 1389537);
    }
}
