pub struct Solution {}

impl Solution {
    pub fn num_of_arrays(n: i32, m: i32, k: i32) -> i32 {
        const M: usize = 1_000_000_007;

        let mut dp = vec![vec![vec![0; m as usize + 1]; k as usize + 1]; n as usize + 1];
        dp[0][0][0] = 1;

        for i in 1..=n as usize {
            for j in 1..=k as usize {
                for x in 0..=m as usize {
                    dp[i][j][x] += dp[i - 1][j][x] * x % M;
                    dp[i][j][x] %= M;
                    for y in x + 1..=m as usize {
                        dp[i][j][y] += dp[i - 1][j - 1][x];
                        dp[i][j][x] %= M;
                    }
                }
            }
        }

        for i in 0..dp.len() {
            for j in 0..dp[0].len() {
                for l in 0..dp[0][0].len() {
                    println!("({}, {}, {}): {}", i, j, l, dp[i][j][l]);
                }
            }
        }

        (1..=m as usize).fold(0, |acc, i| (acc + dp[n as usize][k as usize][i]) % M) as i32
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_1420() {
        assert_eq!(Solution::num_of_arrays(2, 3, 1), 6);
        assert_eq!(Solution::num_of_arrays(5, 2, 3), 0);
        assert_eq!(Solution::num_of_arrays(9, 1, 1), 1);
    }
}
