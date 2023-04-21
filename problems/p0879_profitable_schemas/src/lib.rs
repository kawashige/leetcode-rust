pub struct Solution {}

impl Solution {
    pub fn profitable_schemes(n: i32, min_profit: i32, group: Vec<i32>, profit: Vec<i32>) -> i32 {
        const M: usize = 1_000_000_007;
        let mut dp = vec![vec![vec![0; min_profit as usize + 1]; n as usize + 1]; group.len() + 1];
        dp[0][0][0] = 1;

        for i in 0..group.len() {
            for j in 0..=n as usize {
                for k in 0..dp[i][j].len() {
                    dp[i + 1][j][k] += dp[i][j][k];
                    dp[i + 1][j][k] %= M;
                    if j + group[i] as usize <= n as usize {
                        dp[i + 1][j + group[i] as usize]
                            [(k + profit[i] as usize).min(min_profit as usize)] += dp[i][j][k];
                        dp[i + 1][j + group[i] as usize]
                            [(k + profit[i] as usize).min(min_profit as usize)] %= M;
                    }
                }
            }
        }

        let mut count = 0;
        for i in 0..=n as usize {
            count += dp[group.len()][i][min_profit as usize];
            count %= M;
        }

        count as i32
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_0879() {
        assert_eq!(
            Solution::profitable_schemes(
                100,
                1,
                vec![
                    60, 36, 37, 80, 66, 96, 61, 14, 43, 18, 35, 98, 38, 49, 66, 83, 90, 60, 80, 88,
                    14, 44, 65, 78, 31, 55, 79, 46, 1, 90, 74, 53, 62, 68, 24, 37, 73, 56, 37, 48,
                    86, 51, 56, 66, 51, 72, 29, 34, 96, 57, 84, 13, 99, 69, 47, 45, 55, 58, 31, 60,
                    94, 9, 60, 72, 27, 59, 95, 100, 40, 98, 77, 10, 62, 78, 32, 100, 51, 96, 52,
                    85, 40, 61, 31, 8, 20, 75, 32, 78, 53, 67, 22, 2, 40, 29, 74, 68, 2, 46, 3, 93
                ],
                vec![
                    2, 2, 0, 0, 2, 2, 0, 1, 2, 2, 2, 2, 2, 1, 0, 0, 2, 1, 2, 0, 1, 1, 2, 2, 0, 0,
                    2, 0, 2, 0, 1, 1, 0, 0, 0, 1, 2, 2, 0, 2, 2, 1, 0, 1, 2, 0, 1, 0, 2, 1, 2, 2,
                    2, 0, 1, 1, 0, 0, 0, 2, 1, 2, 1, 0, 2, 1, 1, 1, 0, 1, 1, 2, 2, 0, 1, 1, 1, 1,
                    1, 0, 1, 0, 1, 2, 0, 0, 1, 2, 1, 1, 0, 1, 2, 2, 1, 1, 0, 0, 0, 1
                ]
            ),
            277517
        );
        assert_eq!(
            Solution::profitable_schemes(5, 3, vec![2, 2], vec![2, 3]),
            2
        );
        assert_eq!(
            Solution::profitable_schemes(10, 5, vec![2, 3, 5], vec![6, 7, 8]),
            7
        );
    }
}
