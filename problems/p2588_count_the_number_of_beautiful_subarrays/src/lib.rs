use std::collections::HashMap;

pub struct Solution {}

impl Solution {
    pub fn beautiful_subarrays(nums: Vec<i32>) -> i64 {
        let mut current = 0;
        let mut count = HashMap::new();
        count.insert(0, 1);
        let mut result = 0;

        for i in 0..nums.len() {
            for j in 0..20 {
                if nums[i] & 1 << j != 0 {
                    current ^= 1 << j
                }
            }
            result += count.get(&current).unwrap_or(&0);
            *count.entry(current).or_insert(0) += 1;
        }

        result
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_2588() {
        assert_eq!(Solution::beautiful_subarrays(vec![4, 3, 1, 2, 4]), 2);
        assert_eq!(Solution::beautiful_subarrays(vec![1, 10, 4]), 0);
    }
}
