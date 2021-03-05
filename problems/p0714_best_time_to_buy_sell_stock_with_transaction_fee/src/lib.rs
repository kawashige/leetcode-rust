pub struct Solution {}

impl Solution {
    pub fn max_profit(prices: Vec<i32>, fee: i32) -> i32 {
        let mut cash = 0;
        let mut hold = -prices[0];
        for i in 1..prices.len() {
            cash = std::cmp::max(cash, hold + prices[i] - fee);
            hold = std::cmp::max(hold, cash - prices[i]);
        }
        cash
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_0714() {
        assert_eq!(
            Solution::max_profit(vec![2, 2, 1, 1, 5, 5, 3, 1, 5, 4], 2),
            4
        );
        assert_eq!(Solution::max_profit(vec![1, 3, 2, 8, 4, 9], 2), 8);
        assert_eq!(Solution::max_profit(vec![1, 3, 7, 5, 10, 3], 3), 6);
    }
}
