pub struct Solution {}

impl Solution {
    pub fn coin_change(coins: Vec<i32>, amount: i32) -> i32 {
        let mut dp = vec![vec![100000; amount as usize + 1]; coins.len() + 1];
        dp[0][0] = 0;

        for i in 0..(dp.len() - 1) {
            for j in 0..dp[0].len() {
                dp[i + 1][j] = std::cmp::min(dp[i + 1][j], dp[i][j]);
                if coins[i] as usize <= j {
                    dp[i + 1][j] = std::cmp::min(dp[i + 1][j], dp[i][j - coins[i] as usize] + 1);
                    dp[i + 1][j] =
                        std::cmp::min(dp[i + 1][j], dp[i + 1][j - coins[i] as usize] + 1);
                }
            }
        }
        let result = dp[coins.len()][amount as usize];
        if result == 100000 {
            -1
        } else {
            result
        }
    }
}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn test_0322() {
        assert_eq!(3, Solution::coin_change(vec![1, 2, 5], 11));
        assert_eq!(-1, Solution::coin_change(vec![2], 3));
        assert_eq!(0, Solution::coin_change(vec![1], 0));
        assert_eq!(1, Solution::coin_change(vec![1], 1));
        assert_eq!(2, Solution::coin_change(vec![1], 2));
    }
}
