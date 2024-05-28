use std::collections::HashMap;

pub struct Solution {}

impl Solution {
    pub fn count_good(nums: Vec<i32>, k: i32) -> i64 {
        let mut l = 0;
        let mut pairs = 0;
        let mut map = HashMap::new();
        let mut count = 0;
        for r in 0..nums.len() {
            pairs += map.get(&nums[r]).unwrap_or(&0);
            *map.entry(nums[r]).or_insert(0) += 1;
            while k <= pairs - (map[&nums[l]] - 1) {
                pairs -= map[&nums[l]] - 1;
                *map.get_mut(&nums[l]).unwrap() -= 1;
                l += 1;
            }
            if k <= pairs {
                count += l as i64 + 1;
            }
        }
        count
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_2537() {
        assert_eq!(Solution::count_good(vec![1, 1, 1, 1, 1], 10), 1);
        assert_eq!(Solution::count_good(vec![3, 1, 4, 3, 2, 2, 4], 2), 4);
    }
}
