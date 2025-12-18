pub struct Solution {}

impl Solution {
    pub fn max_profit(prices: Vec<i32>, strategy: Vec<i32>, k: i32) -> i64 {
        let k = k as usize;
        let original: i64 = (0..prices.len())
            .map(|i| prices[i] as i64 * strategy[i] as i64)
            .sum();

        let mut prices_sum = vec![0; prices.len() + 1];
        let mut total_sum = vec![0; prices.len() + 1];
        for i in 0..prices.len() {
            prices_sum[i + 1] = prices_sum[i] + prices[i] as i64;
            total_sum[i + 1] = total_sum[i] + prices[i] as i64 * strategy[i] as i64;
        }

        let mut result = original;
        for i in k - 1..prices.len() {
            let sum = original + prices_sum[i + 1]
                - prices_sum[i + 1 - k / 2]
                - (total_sum[i + 1] - total_sum[i + 1 - k]);
            result = result.max(sum);
        }

        result
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_3562() {
        assert_eq!(Solution::max_profit(vec![4, 2, 8], vec![-1, 0, 1], 2), 10);
        assert_eq!(Solution::max_profit(vec![5, 4, 3], vec![1, 1, 0], 2), 9);
    }
}
