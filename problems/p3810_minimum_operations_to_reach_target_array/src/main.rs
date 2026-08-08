use std::collections::HashSet;

pub struct Solution {}

impl Solution {
    pub fn min_operations(nums: Vec<i32>, target: Vec<i32>) -> i32 {
        let mut set = HashSet::new();
        for i in 0..nums.len() {
            if nums[i] != target[i] {
                set.insert(nums[i]);
            }
        }
        set.len() as i32
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_3810() {
        assert_eq!(Solution::min_operations(vec![1, 2, 3], vec![2, 1, 3]), 2);
        assert_eq!(Solution::min_operations(vec![4, 1, 4], vec![5, 1, 4]), 1);
        assert_eq!(Solution::min_operations(vec![7, 3, 7], vec![5, 5, 9]), 2);
    }
}

fn main() {}
