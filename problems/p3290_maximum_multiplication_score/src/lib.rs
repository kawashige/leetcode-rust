pub struct Solution {}

impl Solution {
    pub fn max_score(a: Vec<i32>, b: Vec<i32>) -> i64 {
        let mut dp = vec![vec![std::i64::MIN; 5]; b.len() + 1];
        dp[0][0] = 0;
        for i in 0..b.len() {
            for j in 0..5 {
                dp[i + 1][j] = dp[i + 1][j].max(dp[i][j]);
                if j < 4 && dp[i][j] != std::i64::MIN {
                    dp[i + 1][j + 1] = dp[i + 1][j + 1].max(dp[i][j] + a[j] as i64 * b[i] as i64);
                }
            }
        }
        *dp.last().unwrap().last().unwrap()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_3290() {
        assert_eq!(
            Solution::max_score(vec![3, 2, 5, 6], vec![2, -6, 4, -5, -3, 2, -7]),
            26
        );
        assert_eq!(
            Solution::max_score(vec![-1, 4, 5, -2], vec![-5, -1, -3, -2, -4]),
            -1
        );
    }
}
