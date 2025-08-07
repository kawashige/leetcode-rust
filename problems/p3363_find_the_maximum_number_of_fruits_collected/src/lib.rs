pub struct Solution {}

impl Solution {
    pub fn max_collected_fruits(fruits: Vec<Vec<i32>>) -> i32 {
        let n = fruits.len();
        let mut result = 0;
        for i in 0..n {
            result += fruits[i][i];
        }

        let mut dp = vec![vec![std::i32::MIN; n / 2]; n - 1];
        dp[0][0] = fruits[n - 1][0];
        for i in 1..dp.len() {
            for j in 0..dp[0].len() {
                let mut prev = dp[i - 1][j];
                if 0 < j {
                    prev = prev.max(dp[i - 1][j - 1]);
                }
                if j < dp[i].len() - 1 {
                    prev = prev.max(dp[i - 1][j + 1])
                }
                dp[i][j] = prev + fruits[n - 1 - j][i];
            }
        }

        let mut dp2 = vec![vec![std::i32::MIN; n / 2]; n - 1];
        dp2[0][0] = fruits[0][n - 1];
        for i in 1..dp2.len() {
            for j in 0..dp2[0].len() {
                let mut prev = dp2[i - 1][j];
                if 0 < j {
                    prev = prev.max(dp2[i - 1][j - 1]);
                }
                if j < dp2[i].len() - 1 {
                    prev = prev.max(dp2[i - 1][j + 1])
                }
                dp2[i][j] = prev + fruits[i][n - 1 - j];
            }
        }

        result + dp[dp.len() - 1][0] + dp2[dp2.len() - 1][0]
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_3363() {
        assert_eq!(
            Solution::max_collected_fruits(vec![
                vec![1, 2, 3, 4],
                vec![5, 6, 8, 7],
                vec![9, 10, 11, 12],
                vec![13, 14, 15, 16]
            ]),
            100
        );
        assert_eq!(
            Solution::max_collected_fruits(vec![vec![1, 1], vec![1, 1]]),
            4
        );
    }
}
