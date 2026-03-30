use std::collections::BTreeSet;

pub struct Solution {}

impl Solution {
    fn neighbors(tree: &BTreeSet<i32>, val: i32) -> (Option<&i32>, Option<&i32>) {
        use std::ops::Bound::*;

        let mut before = tree.range((Unbounded, Included(val)));
        let mut after = tree.range((Included(val), Unbounded));

        (before.next_back(), after.next())
    }

    pub fn min_absolute_difference(nums: Vec<i32>, x: i32) -> i32 {
        if x == 0 {
            return 0;
        }
        let x = x as usize;
        let mut set = BTreeSet::new();
        let mut result = 1_000_000_000;
        for i in 0..nums.len() {
            if x <= i {
                set.insert(nums[i - x]);
            }
            let (prev, next) = Self::neighbors(&set, nums[i]);
            if let Some(v) = prev {
                result = result.min((nums[i] - v).abs());
            }
            if let Some(v) = next {
                result = result.min((nums[i] - v).abs());
            }
        }

        result
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_2817() {
        assert_eq!(Solution::min_absolute_difference(vec![4, 3, 2, 4], 2), 0);
        assert_eq!(
            Solution::min_absolute_difference(vec![5, 3, 2, 10, 15], 1),
            1
        );
        assert_eq!(Solution::min_absolute_difference(vec![1, 2, 3, 4], 3), 3);
    }
}
