pub struct Solution {}

impl Solution {
    pub fn num_rolls_to_target(n: i32, k: i32, target: i32) -> i32 {
        const M: i32 = 1_000_000_007;

        let mut dp = vec![vec![0; target as usize + 1]; n as usize + 1];
        dp[0][0] = 1;

        for i in 0..(dp.len() - 1) {
            for j in 0..dp[0].len() {
                if dp[i][j] > 0 {
                    for l in 1..=(k as usize) {
                        if (target as usize) < j + l {
                            break;
                        }
                        dp[i + 1][j + l] = (dp[i + 1][j + l] + dp[i][j]) % M;
                    }
                }
            }
        }

        dp[n as usize][target as usize]
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_1155() {
        assert_eq!(Solution::num_rolls_to_target(1, 6, 3), 1);
        assert_eq!(Solution::num_rolls_to_target(2, 6, 7), 6);
        assert_eq!(Solution::num_rolls_to_target(30, 30, 500), 222616187);
    }
}
