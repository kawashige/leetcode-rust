pub struct Solution {}

impl Solution {
    pub fn find_integers(n: i32) -> i32 {
        let l = (0..32).rev().find(|i| n & 1 << i > 0).unwrap();

        // dp[i桁目][smaller][0 or 1]
        let mut dp = vec![vec![vec![0; 2]; 2]; l + 1];
        dp[l][1][0] = 1;
        dp[l][0][1] = 1;

        for i in (0..(dp.len() - 1)).rev() {
            if n & 1 << i == 0 {
                dp[i][0][0] = dp[i + 1][0][0] + dp[i + 1][0][1];
                dp[i][1][0] = dp[i + 1][1][0] + dp[i + 1][1][1];
                dp[i][1][1] = dp[i + 1][1][0];
            } else {
                dp[i][0][1] = dp[i + 1][0][0];
                dp[i][1][0] = dp[i + 1][0][0] + dp[i + 1][0][1] + dp[i + 1][1][0] + dp[i + 1][1][1];
                dp[i][1][1] = dp[i + 1][1][0];
            }
        }

        dp[0].iter().map(|v| v.iter().sum::<i32>()).sum::<i32>()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_day25() {
        assert_eq!(Solution::find_integers(5), 5);
        assert_eq!(Solution::find_integers(1), 2);
        assert_eq!(Solution::find_integers(2), 3);
    }
}
