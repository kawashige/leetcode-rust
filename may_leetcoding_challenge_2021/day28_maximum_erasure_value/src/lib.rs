use std::collections::HashSet;

pub struct Solution {}

impl Solution {
    pub fn maximum_unique_subarray(nums: Vec<i32>) -> i32 {
        let mut max = 0;
        let mut sum = 0;
        let mut i = 0;
        let mut set = HashSet::new();

        for j in 0..nums.len() {
            if set.contains(&nums[j]) {
                while nums[i] != nums[j] {
                    sum -= nums[i];
                    set.remove(&nums[i]);
                    i += 1;
                }
                i += 1;
            } else {
                set.insert(nums[j]);
                sum += nums[j];
                max = std::cmp::max(max, sum);
            }
        }

        std::cmp::max(max, sum)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_day28() {
        assert_eq!(Solution::maximum_unique_subarray(vec![4, 2, 4, 5, 6]), 17);
        assert_eq!(
            Solution::maximum_unique_subarray(vec![5, 2, 1, 2, 5, 2, 1, 2, 5]),
            8
        );
    }
}
