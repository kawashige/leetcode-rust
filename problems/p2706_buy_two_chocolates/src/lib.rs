pub struct Solution {}

impl Solution {
    pub fn buy_choco(prices: Vec<i32>, money: i32) -> i32 {
        let mut prices = prices;
        prices.sort_unstable();
        if prices[0] + prices[1] <= money {
            money - prices[0] - prices[1]
        } else {
            money
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_2706() {
        assert_eq!(Solution::buy_choco(vec![1, 2, 2], 3), 0);
        assert_eq!(Solution::buy_choco(vec![3, 2, 3], 3), 3);
    }
}
