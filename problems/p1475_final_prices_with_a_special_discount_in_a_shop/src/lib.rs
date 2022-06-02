use std::collections::BinaryHeap;

pub struct Solution {}

impl Solution {
    pub fn final_prices(mut prices: Vec<i32>) -> Vec<i32> {
        let mut heap = BinaryHeap::new();

        for i in 0..prices.len() {
            while let Some((v, j)) = heap.pop() {
                if v < prices[i] {
                    heap.push((v, j));
                    break;
                }
                prices[j] -= prices[i];
            }
            heap.push((prices[i], i));
        }

        prices
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_1475() {
        assert_eq!(
            Solution::final_prices(vec![8, 4, 6, 2, 3]),
            vec![4, 2, 4, 2, 3]
        );
        assert_eq!(
            Solution::final_prices(vec![1, 2, 3, 4, 5]),
            vec![1, 2, 3, 4, 5]
        );
        assert_eq!(Solution::final_prices(vec![10, 1, 1, 6]), vec![9, 0, 1, 6]);
    }
}
