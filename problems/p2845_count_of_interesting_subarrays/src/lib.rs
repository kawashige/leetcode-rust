use std::collections::HashMap;

pub struct Solution {}

impl Solution {
    pub fn count_interesting_subarrays(nums: Vec<i32>, modulo: i32, k: i32) -> i64 {
        let mut counts = HashMap::new();
        let mut count = 0;
        let mut result = 0;
        counts.insert(0, 1);

        for i in 0..nums.len() {
            if nums[i] % modulo == k {
                count = (count + 1) % modulo;
            }
            result += *counts.get(&((modulo + count - k) % modulo)).unwrap_or(&0) as i64;
            *counts.entry(count).or_insert(0) += 1;
        }

        result
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_2845() {
        assert_eq!(
            Solution::count_interesting_subarrays(vec![3, 2, 4], 2, 1),
            3
        );
        assert_eq!(
            Solution::count_interesting_subarrays(vec![3, 1, 9, 6], 3, 0),
            2
        );
    }
}
