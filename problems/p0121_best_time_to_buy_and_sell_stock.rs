pub struct Solution {}

impl Solution {
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        let mut profit = 0;
        if prices.len() > 0 {
            let mut min = prices[0];
            for i in 1..prices.len() {
                if min < prices[i] {
                    profit = std::cmp::max(profit, prices[i] - min);
                } else {
                    min = prices[i];
                }
            }
        }
        profit
    }
}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn test_0121() {
        assert_eq!(5, Solution::max_profit(vec![7, 1, 5, 3, 6, 4]));
        assert_eq!(0, Solution::max_profit(vec![7, 6, 4, 3, 1]));
        assert_eq!(0, Solution::max_profit(vec![]));
    }
}
