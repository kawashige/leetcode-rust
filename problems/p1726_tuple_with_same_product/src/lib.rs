use std::collections::HashMap;

pub struct Solution {}

impl Solution {
    pub fn tuple_same_product(nums: Vec<i32>) -> i32 {
        let mut count = 0;
        let mut products = HashMap::new();

        for i in 0..nums.len() {
            for j in i + 1..nums.len() {
                let product = nums[i] * nums[j];
                count += products.get(&product).unwrap_or(&0) * 8;
                *products.entry(product).or_insert(0) += 1;
            }
        }

        count
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_1726() {
        assert_eq!(Solution::tuple_same_product(vec![2, 3, 4, 6]), 8);
        assert_eq!(Solution::tuple_same_product(vec![1, 2, 4, 5, 10]), 16);
    }
}
