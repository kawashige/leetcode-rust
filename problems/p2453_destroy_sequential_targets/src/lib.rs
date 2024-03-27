use std::collections::{HashMap, HashSet};

pub struct Solution {}

impl Solution {
    pub fn destroy_targets(nums: Vec<i32>, space: i32) -> i32 {
        let mut nums = nums;
        nums.sort_unstable();

        let mut count = HashMap::new();
        for num in &nums {
            *count.entry(num % space).or_insert(0) += 1;
        }

        let max_count = *count.values().max().unwrap();
        let targets = count
            .into_iter()
            .filter(|(_, v)| v == &max_count)
            .map(|(k, _)| k)
            .collect::<HashSet<_>>();

        nums.into_iter()
            .find(|num| targets.contains(&(num % space)))
            .unwrap()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_2453() {
        assert_eq!(Solution::destroy_targets(vec![3, 7, 8, 1, 1, 5], 2), 1);
        assert_eq!(Solution::destroy_targets(vec![1, 3, 5, 2, 4, 6], 2), 1);
        assert_eq!(Solution::destroy_targets(vec![6, 2, 5], 100), 2);
        assert_eq!(Solution::destroy_targets(vec![691], 4), 691);
    }
}
