pub struct Solution {}

impl Solution {
    pub fn knight_dialer(n: i32) -> i32 {
        if n == 1 {
            return 10;
        }

        let n = n as usize;
        const M: i32 = 1_000_000_007;

        let mut dp = vec![vec![0; 10]; n];
        dp[0] = vec![1; 10];

        for i in 1..n {
            dp[i][0] = (dp[i - 1][4] + dp[i - 1][6]) % M;
            dp[i][1] = (dp[i - 1][6] + dp[i - 1][8]) % M;
            dp[i][2] = (dp[i - 1][7] + dp[i - 1][9]) % M;
            dp[i][3] = (dp[i - 1][4] + dp[i - 1][8]) % M;
            dp[i][4] = (((dp[i - 1][0] + dp[i - 1][3]) % M) + dp[i - 1][9]) % M;
            dp[i][6] = (((dp[i - 1][0] + dp[i - 1][1]) % M) + dp[i - 1][7]) % M;
            dp[i][7] = (dp[i - 1][2] + dp[i - 1][6]) % M;
            dp[i][8] = (dp[i - 1][1] + dp[i - 1][3]) % M;
            dp[i][9] = (dp[i - 1][2] + dp[i - 1][4]) % M;
        }

        dp[n - 1].iter().fold(0, |acc, x| (acc + x) % M)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_0935() {
        assert_eq!(Solution::knight_dialer(1), 10);
        assert_eq!(Solution::knight_dialer(2), 20);
        assert_eq!(Solution::knight_dialer(3), 46);
        assert_eq!(Solution::knight_dialer(4), 104);
        assert_eq!(Solution::knight_dialer(3131), 136006598);
    }
}
