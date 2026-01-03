pub struct Solution {}

impl Solution {
    pub fn num_of_ways(n: i32) -> i32 {
        const M: usize = 1_000_000_007;
        let mut dp = vec![vec![vec![vec![0; 3]; 3]; 3]; n as usize];
        for i in 0..3 {
            for j in 0..3 {
                for k in 0..3 {
                    if i != j && j != k {
                        dp[0][i][j][k] = 1;
                    }
                }
            }
        }

        for r in 1..n as usize {
            for i in 0..3 {
                for j in 0..3 {
                    for k in 0..3 {
                        for x in 0..3 {
                            if x == i {
                                continue;
                            }
                            for y in 0..3 {
                                if x == y || j == y {
                                    continue;
                                }
                                for z in 0..3 {
                                    if y == z || k == z {
                                        continue;
                                    }
                                    dp[r][x][y][z] = (dp[r][x][y][z] + dp[r - 1][i][j][k]) % M;
                                }
                            }
                        }
                    }
                }
            }
        }

        let mut result = 0;
        for i in 0..3 {
            for j in 0..3 {
                for k in 0..3 {
                    result = (result + dp[n as usize - 1][i][j][k]) % M
                }
            }
        }
        result as i32
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_1411() {
        assert_eq!(Solution::num_of_ways(1), 12);
        assert_eq!(Solution::num_of_ways(5000), 30228214);
    }
}
