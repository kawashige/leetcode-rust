pub struct Solution {}

impl Solution {
    pub fn count_house_placements(n: i32) -> i32 {
        const M: usize = 1_000_000_007;

        let mut dp = vec![[0; 2]; n as usize];
        dp[0][0] = 1;
        dp[0][1] = 1;

        for i in 1..n as usize {
            dp[i][0] = dp[i - 1][0] + dp[i - 1][1];
            dp[i][1] = dp[i - 1][0];
            dp[i][0] %= M;
            dp[i][1] %= M;
        }

        ((dp[n as usize - 1][0] + dp[n as usize - 1][1])
            * (dp[n as usize - 1][0] + dp[n as usize - 1][1])
            % M) as i32
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_2320() {
        assert_eq!(Solution::count_house_placements(1), 4);
        assert_eq!(Solution::count_house_placements(2), 9);
    }
}
