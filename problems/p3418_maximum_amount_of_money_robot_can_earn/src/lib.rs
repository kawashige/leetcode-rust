pub struct Solution {}

impl Solution {
    pub fn maximum_amount(coins: Vec<Vec<i32>>) -> i32 {
        let mut dp = vec![vec![vec![std::i32::MIN; 3]; coins[0].len()]; coins.len()];
        dp[0][0][1] = 0;
        dp[0][0][2] = coins[0][0];

        for i in 0..coins.len() {
            for j in 0..coins[0].len() {
                for k in 0..3 {
                    if 0 < i && dp[i - 1][j][k] != std::i32::MIN {
                        dp[i][j][k] = coins[i][j] + dp[i - 1][j][k];
                    }
                    if 0 < j && dp[i][j - 1][k] != std::i32::MIN {
                        dp[i][j][k] = dp[i][j][k].max(coins[i][j] + dp[i][j - 1][k]);
                    }
                    if k != 2 {
                        if 0 < i && dp[i - 1][j][k + 1] != std::i32::MIN {
                            dp[i][j][k] = dp[i][j][k].max(dp[i - 1][j][k + 1]);
                        }
                        if 0 < j && dp[i][j - 1][k + 1] != std::i32::MIN {
                            dp[i][j][k] = dp[i][j][k].max(dp[i][j - 1][k + 1]);
                        }
                    }
                }
            }
        }

        *dp[dp.len() - 1][dp[0].len() - 1].iter().max().unwrap()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_3418() {
        assert_eq!(Solution::maximum_amount(vec![vec![-4]]), 0);
        assert_eq!(
            Solution::maximum_amount(vec![vec![0, 1, -1], vec![1, -2, 3], vec![2, -3, 4]]),
            8
        );
        assert_eq!(
            Solution::maximum_amount(vec![vec![10, 10, 10], vec![10, 10, 10]]),
            40
        );
    }
}
