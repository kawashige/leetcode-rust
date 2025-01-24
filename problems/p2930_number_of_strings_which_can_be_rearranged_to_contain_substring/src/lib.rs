pub struct Solution {}

impl Solution {
    pub fn string_count(n: i32) -> i32 {
        const M: usize = 1_000_000_007;

        let mut dp = vec![vec![vec![0; 2]; 3]; 2];
        dp[0][0][0] = 1;
        for _ in 0..n {
            let mut new_dp = vec![vec![vec![0; 2]; 3]; 2];
            for l in 0..2 {
                for e in 0..3 {
                    for t in 0..2 {
                        if l < 1 {
                            new_dp[l + 1][e][t] += dp[l][e][t];
                            new_dp[l + 1][e][t] %= M;
                        }
                        if e < 2 {
                            new_dp[l][e + 1][t] += dp[l][e][t];
                            new_dp[l][e + 1][t] %= M;
                        }
                        if t < 1 {
                            new_dp[l][e][t + 1] += dp[l][e][t];
                            new_dp[l][e][t + 1] %= M;
                        }

                        new_dp[l][e][t] += dp[l][e][t]
                            * (26 - ((l != 1) as usize + (e != 2) as usize + (t != 1) as usize));
                        new_dp[l][e][t] %= M;
                    }
                }
            }
            dp = new_dp;
        }

        dp[1][2][1] as i32
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_2930() {
        assert_eq!(Solution::string_count(4), 12);
        assert_eq!(Solution::string_count(10), 83943898);
    }
}
