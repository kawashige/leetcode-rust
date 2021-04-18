pub struct Solution {}

impl Solution {
    pub fn num_tilings(n: i32) -> i32 {
        let n = n as usize;
        let m = 1_000_000_007;

        let mut dp = vec![vec![0; n + 1]; n + 1];
        dp[0][0] = 1;
        dp[1][1] = 1;

        for i in 1..=n {
            for j in (i - 1)..=std::cmp::min(i + 1, n) {
                if i == 1 && j == 1 {
                    continue;
                }

                let mut tmp = 0;

                if i == j {
                    tmp = (tmp + dp[i - 1][j - 1]) % m;
                    if i > 1 {
                        tmp = (tmp + dp[i - 2][j - 1]) % m;
                    }
                    if j > 1 {
                        tmp = (tmp + dp[i - 1][j - 2]) % m;
                    }
                    if i > 1 && j > 1 {
                        tmp = (tmp + dp[i - 2][j - 2]) % m;
                    }
                } else {
                    if i > 1 {
                        tmp = (tmp + dp[i - 2][j]) % m;
                    }
                    if j > 1 {
                        tmp = (tmp + dp[i][j - 2]) % m;
                    }

                    if i == j + 1 && i > 1 {
                        tmp = (tmp + dp[i - 2][j - 1]) % m;
                    }
                    if i + 1 == j && j > 1 {
                        tmp = (tmp + dp[i - 1][j - 2]) % m;
                    }
                }

                dp[i][j] = tmp;
            }
        }

        dp[n][n]
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_0790() {
        assert_eq!(Solution::num_tilings(5), 24);
        assert_eq!(Solution::num_tilings(2), 2);
        assert_eq!(Solution::num_tilings(3), 5);
    }
}
