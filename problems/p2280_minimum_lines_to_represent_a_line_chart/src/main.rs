pub struct Solution {}

impl Solution {
    pub fn minimum_lines(stock_prices: Vec<Vec<i32>>) -> i32 {
        if stock_prices.len() == 1 {
            return 0;
        }
        let mut stock_prices = stock_prices;
        stock_prices.sort_unstable();

        let mut lines = 1;

        for i in 2..stock_prices.len() {
            if (stock_prices[i][1] - stock_prices[i - 1][1]) as i64
                * (stock_prices[i - 1][0] - stock_prices[i - 2][0]) as i64
                != (stock_prices[i - 1][1] - stock_prices[i - 2][1]) as i64
                    * (stock_prices[i][0] - stock_prices[i - 1][0]) as i64
            {
                lines += 1;
            }
        }

        lines
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_2280() {
        assert_eq!(
            Solution::minimum_lines(vec![
                vec![1, 7],
                vec![2, 6],
                vec![3, 5],
                vec![4, 4],
                vec![5, 4],
                vec![6, 3],
                vec![7, 2],
                vec![8, 1]
            ]),
            3
        );
        assert_eq!(
            Solution::minimum_lines(vec![vec![3, 4], vec![1, 2], vec![7, 8], vec![2, 3]]),
            1
        );
    }
}
