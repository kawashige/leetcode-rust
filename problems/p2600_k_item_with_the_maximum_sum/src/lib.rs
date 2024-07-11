pub struct Solution {}

impl Solution {
    pub fn k_items_with_maximum_sum(
        num_ones: i32,
        num_zeros: i32,
        num_neg_ones: i32,
        k: i32,
    ) -> i32 {
        let num = [num_ones, num_zeros, num_neg_ones];
        let item = [1, 0, -1];

        let mut k = k;
        let mut sum = 0;
        for i in 0..num.len() {
            sum += item[i] * k.min(num[i]);
            k -= k.min(num[i]);
        }

        sum
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_2600() {
        assert_eq!(Solution::k_items_with_maximum_sum(3, 2, 0, 2), 2);
        assert_eq!(Solution::k_items_with_maximum_sum(3, 2, 0, 4), 3);
    }
}
