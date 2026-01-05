pub struct Solution {}

impl Solution {
    pub fn min_cost(n: i32, cost: Vec<Vec<i32>>) -> i64 {
        let n = n as usize;
        let mut dp = vec![vec![vec![std::i64::MAX; 3]; 3]; n / 2];

        for i in 0..3 {
            for j in 0..3 {
                if i != j {
                    dp[0][i][j] = (cost[0][i] + cost[n - 1][j]) as i64;
                }
            }
        }

        for i in 1..n / 2 {
            for j in 0..3 {
                for k in 0..3 {
                    if j == k {
                        continue;
                    }
                    for l in 0..3 {
                        if j == l {
                            continue;
                        }
                        for m in 0..3 {
                            if k == m || l == m {
                                continue;
                            }
                            dp[i][j][k] = dp[i][j][k]
                                .min(dp[i - 1][l][m] + (cost[i][j] + cost[n - 1 - i][k]) as i64);
                        }
                    }
                }
            }
        }

        let mut result = std::i64::MAX;
        for i in 0..3 {
            for j in 0..3 {
                result = result.min(dp[n / 2 - 1][i][j]);
            }
        }
        result
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_3429() {
        assert_eq!(
            Solution::min_cost(
                4,
                vec![vec![3, 5, 7], vec![6, 2, 9], vec![4, 8, 1], vec![7, 3, 5]]
            ),
            9
        );
        assert_eq!(
            Solution::min_cost(
                6,
                vec![
                    vec![2, 4, 6],
                    vec![5, 3, 8],
                    vec![7, 1, 9],
                    vec![4, 6, 2],
                    vec![3, 5, 7],
                    vec![8, 2, 4]
                ]
            ),
            18
        );
    }
}
