pub struct Solution {}

impl Solution {
    pub fn number_of_stable_arrays(zero: i32, one: i32, limit: i32) -> i32 {
        const M: i32 = 1_000_000_007;
        let zero = zero as usize;
        let one = one as usize;
        let limit = limit as usize;

        let mut dp = vec![vec![vec![0; 2]; one + 1]; zero + 1];

        for i in 0..=zero {
            for j in 0..=one {
                for k in 0..2 {
                    if i == 0 {
                        if k == 0 || limit < j {
                            dp[i][j][k] = 0;
                        } else {
                            dp[i][j][k] = 1;
                        }
                    } else if j == 0 {
                        if k == 1 || limit < i {
                            dp[i][j][k] = 0;
                        } else {
                            dp[i][j][k] = 1;
                        }
                    } else if k == 0 {
                        dp[i][j][k] = dp[i - 1][j][0] + dp[i - 1][j][1];
                        if limit < i {
                            dp[i][j][k] -= dp[i - limit - 1][j][1];
                        }
                    } else {
                        dp[i][j][k] = dp[i][j - 1][0] + dp[i][j - 1][1];
                        if limit < j {
                            dp[i][j][k] -= dp[i][j - limit - 1][0];
                        }
                    }
                    dp[i][j][k] %= M;
                    if dp[i][j][k] < 0 {
                        dp[i][j][k] += M;
                    }
                }
            }
        }

        (dp[zero][one][0] + dp[zero][one][1]) % M
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_3130() {
        assert_eq!(Solution::number_of_stable_arrays(1, 1, 2), 2);
        assert_eq!(Solution::number_of_stable_arrays(1, 2, 1), 1);
        assert_eq!(Solution::number_of_stable_arrays(3, 3, 2), 14);
    }
}
