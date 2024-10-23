use std::collections::HashMap;

pub struct Solution {}

impl Solution {
    pub fn minimum_index(nums: Vec<i32>) -> i32 {
        if nums.len() == 1 {
            return -1;
        }
        let mut counts = HashMap::new();
        for num in &nums {
            *counts.entry(*num).or_insert(0) += 1;
        }
        let mut counts = counts.into_iter().collect::<Vec<_>>();
        counts.sort_unstable_by_key(|(_, c)| -1 * *c as i32);

        let (dominant, count) = counts[0];
        let mut c = 0;
        for i in 0..nums.len() {
            if nums[i] == dominant {
                c += 1;
            }
            if i + 1 < c * 2 && nums.len() - i - 1 < (count - c) * 2 {
                return i as i32;
            }
        }
        -1
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_2780() {
        assert_eq!(Solution::minimum_index(vec![1, 2, 2, 2]), 2);
        assert_eq!(
            Solution::minimum_index(vec![2, 1, 3, 1, 1, 1, 7, 1, 2, 1]),
            4
        );
        assert_eq!(Solution::minimum_index(vec![3, 3, 3, 3, 7, 2, 2]), -1);
    }
}
