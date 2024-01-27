use std::collections::HashMap;

pub struct Solution {}

impl Solution {
    pub fn count_bad_pairs(nums: Vec<i32>) -> i64 {
        let mut map = HashMap::new();
        let mut count = 0;
        for i in 0..nums.len() {
            count += (i - map.get(&(nums[i] - i as i32)).unwrap_or(&0)) as i64;
            *map.entry(nums[i] - i as i32).or_insert(0) += 1;
        }
        count
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_2364() {
        assert_eq!(Solution::count_bad_pairs(vec![4, 1, 3, 3]), 5);
        assert_eq!(Solution::count_bad_pairs(vec![1, 2, 3, 4, 5]), 0);
    }
}
