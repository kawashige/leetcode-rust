pub struct Solution {}

impl Solution {
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        if prices.len() < 2 {
            return 0;
        }
        let mut dp = vec![0; prices.len()];
        for i in 1..prices.len() {
            let mut max = 0;
            for j in 0..i {
                if prices[j] < prices[i] {
                    let mut profit = prices[i] - prices[j];
                    if 2 < j {
                        profit += dp[j - 2];
                    }
                    max = std::cmp::max(max, profit);
                }
            }
            dp[i] = std::cmp::max(max, dp[i - 1]);
        }
        *dp.last().unwrap()
    }
}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn test_0309() {
        assert_eq!(3, Solution::max_profit(vec![1, 4, 2]));
        assert_eq!(3, Solution::max_profit(vec![1, 2, 3, 0, 2]))
    }
}
