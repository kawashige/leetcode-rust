pub struct Solution {}

impl Solution {
    pub fn max_value_of_coins(piles: Vec<Vec<i32>>, k: i32) -> i32 {
        let mut dp = vec![vec![0; k as usize + 1]; piles.len() + 1];

        for i in 0..piles.len() {
            for j in 0..dp[i + 1].len() {
                dp[i + 1][j] = dp[i][j];
            }

            let mut sum = 0;
            for j in 0..piles[i].len().min(k as usize) {
                sum += piles[i][j];
                for l in 0..k as usize - j {
                    if l == 0 || 0 < dp[i][l] {
                        dp[i + 1][l + j + 1] = dp[i + 1][l + j + 1].max(dp[i][l] + sum);
                    }
                }
            }
        }

        dp.last().unwrap()[k as usize]
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_2218() {
        assert_eq!(
            Solution::max_value_of_coins(vec![vec![1, 100, 3], vec![7, 8, 9]], 2),
            101
        );
        assert_eq!(
            Solution::max_value_of_coins(
                vec![
                    vec![100],
                    vec![100],
                    vec![100],
                    vec![100],
                    vec![100],
                    vec![100],
                    vec![1, 1, 1, 1, 1, 1, 700]
                ],
                7
            ),
            706
        );
    }
}
