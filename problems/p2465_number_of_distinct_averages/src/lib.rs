use std::collections::HashSet;

pub struct Solution {}

impl Solution {
    pub fn distinct_averages(nums: Vec<i32>) -> i32 {
        let mut set = HashSet::new();
        let mut nums = nums;
        nums.sort_unstable();

        for i in 0..nums.len() / 2 {
            set.insert(nums[i] + nums[nums.len() - 1 - i]);
        }

        set.len() as i32
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_2465() {
        assert_eq!(Solution::distinct_averages(vec![4, 1, 4, 0, 3, 5]), 2);
        assert_eq!(Solution::distinct_averages(vec![1, 100]), 1);
    }
}
