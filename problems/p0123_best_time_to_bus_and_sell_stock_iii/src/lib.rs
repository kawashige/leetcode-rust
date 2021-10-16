pub struct Solution {}

impl Solution {
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        let mut left = vec![0; prices.len()];
        let mut min = prices[0];
        for i in 1..prices.len() {
            left[i] = left[i - 1];
            if min < prices[i] {
                left[i] = left[i].max(prices[i] - min);
            } else {
                min = prices[i];
            }
        }

        let mut right = vec![0; prices.len()];
        let mut max = prices[prices.len() - 1];
        for i in (0..(prices.len() - 1)).rev() {
            right[i] = right[i + 1];
            if prices[i] < max {
                right[i] = right[i].max(max - prices[i]);
            } else {
                max = prices[i];
            }
        }

        (1..prices.len())
            .map(|i| left[i - 1] + right[i])
            .max()
            .unwrap_or(0)
            .max(left[prices.len() - 1])
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_0123() {
        assert_eq!(Solution::max_profit(vec![3, 3, 5, 0, 0, 3, 1, 4]), 6);
        assert_eq!(Solution::max_profit(vec![1, 2, 3, 4, 5]), 4);
        assert_eq!(Solution::max_profit(vec![7, 6, 4, 3, 1]), 0);
        assert_eq!(Solution::max_profit(vec![1]), 0);
    }
}
