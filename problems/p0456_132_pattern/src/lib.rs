use std::collections::BTreeSet;

pub struct Solution {}

impl Solution {
    pub fn find132pattern(nums: Vec<i32>) -> bool {
        let mut left_min = vec![std::i32::MAX; nums.len()];
        left_min[0] = nums[0];
        for i in 1..nums.len() {
            left_min[i] = nums[i].min(left_min[i - 1]);
        }

        let mut right_nums = BTreeSet::new();
        right_nums.insert(*nums.last().unwrap());
        for i in (1..nums.len() - 1).rev() {
            if left_min[i - 1] + 1 < nums[i]
                && right_nums
                    .range(left_min[i - 1] + 1..nums[i])
                    .next()
                    .is_some()
            {
                return true;
            }
            right_nums.insert(nums[i]);
        }

        false
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_0456() {
        assert!(Solution::find132pattern(vec![
            1, 3, 2, 4, 5, 6, 7, 8, 9, 10
        ]));
        assert!(Solution::find132pattern(vec![3, 5, 0, 3, 4]));
        assert!(!Solution::find132pattern(vec![1, 2, 3, 4]));
        assert!(Solution::find132pattern(vec![3, 1, 4, 2]));
        assert!(Solution::find132pattern(vec![-1, 3, 2, 0]));
    }
}
