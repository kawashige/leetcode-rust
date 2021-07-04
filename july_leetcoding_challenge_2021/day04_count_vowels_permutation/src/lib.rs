pub struct Solution {}

impl Solution {
    pub fn count_vowel_permutation(n: i32) -> i32 {
        const M: i32 = 1_000_000_007;

        let mut dp = vec![vec![0; 5]; n as usize];
        (0..5).for_each(|i| dp[0][i] = 1);

        for i in 1..(n as usize) {
            dp[i][0] = dp[i - 1][1];
            dp[i][1] = (dp[i - 1][0] + dp[i - 1][2]) % M;
            dp[i][2] = ((dp[i - 1][0] + dp[i - 1][1]) % M + (dp[i - 1][3] + dp[i - 1][4]) % M) % M;
            dp[i][3] = (dp[i - 1][2] + dp[i - 1][4]) % M;
            dp[i][4] = dp[i - 1][0];
        }

        dp.last()
            .unwrap()
            .into_iter()
            .fold(0, |acc, x| (acc + x) % M)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_day04() {
        assert_eq!(Solution::count_vowel_permutation(1), 5);
        assert_eq!(Solution::count_vowel_permutation(2), 10);
        assert_eq!(Solution::count_vowel_permutation(5), 68);
    }
}
