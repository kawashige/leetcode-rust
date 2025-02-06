pub struct Solution {}

impl Solution {
    pub fn minimum_coins(prices: Vec<i32>) -> i32 {
        let mut dp = vec![std::i32::MAX; prices.len() + 1];
        dp[0] = 0;

        for i in 0..prices.len() {
            let r = (i + i + 2).min(prices.len());
            for j in i..=r {
                if dp[j] != std::i32::MAX {
                    dp[r] = dp[r].min(dp[j] + prices[i]);
                }
            }
        }

        dp[dp.len() - 1]
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_2944() {
        assert_eq!(Solution::minimum_coins(vec![3, 1, 2]), 4);
        assert_eq!(Solution::minimum_coins(vec![1, 10, 1, 1]), 2);
        assert_eq!(
            Solution::minimum_coins(vec![26, 18, 6, 12, 49, 7, 45, 45]),
            39
        );
    }
}
