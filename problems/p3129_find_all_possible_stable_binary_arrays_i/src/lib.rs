pub struct Solution {}

impl Solution {
    pub fn number_of_stable_arrays(zero: i32, one: i32, limit: i32) -> i32 {
        const M: usize = 1_000_000_007;

        let mut dp = vec![vec![vec![0; 2]; zero as usize + 1]; zero as usize + one as usize + 1];
        dp[0][0][0] = 1;
        dp[0][0][1] = 1;

        for i in 1..=(zero + one) as usize {
            for j in 0..=zero as usize {
                for k in 1..=limit as usize {
                    if i - 1 + k > (zero + one) as usize {
                        continue;
                    }
                    if i - 1 + k <= one as usize + j {
                        dp[i - 1 + k][j][1] += dp[i - 1][j][0];
                        dp[i - 1 + k][j][1] %= M;
                    }
                    if j + k <= zero as usize {
                        dp[i - 1 + k][j + k][0] += dp[i - 1][j][1];
                        dp[i - 1 + k][j + k][0] += M;
                    }
                }
            }
        }

        dp.into_iter()
            .last()
            .unwrap()
            .into_iter()
            .last()
            .unwrap()
            .into_iter()
            .fold(0, |acc, x| (acc + x) % M) as i32
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_3129() {
        assert_eq!(Solution::number_of_stable_arrays(1, 1, 2), 2);
        assert_eq!(Solution::number_of_stable_arrays(1, 2, 1), 1);
        assert_eq!(Solution::number_of_stable_arrays(3, 3, 2), 14);
    }
}
