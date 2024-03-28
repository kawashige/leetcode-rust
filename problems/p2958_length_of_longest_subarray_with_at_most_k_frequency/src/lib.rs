use std::collections::HashMap;

pub struct Solution {}

impl Solution {
    pub fn max_subarray_length(nums: Vec<i32>, k: i32) -> i32 {
        let mut l = 0;
        let mut result = 1;
        let mut count = HashMap::new();
        count.insert(nums[0], 1);

        for r in 1..nums.len() {
            *count.entry(nums[r]).or_insert(0) += 1;
            while k < count[&nums[r]] {
                *count.get_mut(&nums[l]).unwrap() -= 1;
                l += 1;
            }
            result = result.max(r - l + 1)
        }

        result as i32
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_2958() {
        assert_eq!(
            Solution::max_subarray_length(vec![1, 2, 3, 1, 2, 3, 1, 2], 2),
            6
        );
        assert_eq!(
            Solution::max_subarray_length(vec![1, 2, 1, 2, 1, 2, 1, 2], 1),
            2
        );
        assert_eq!(
            Solution::max_subarray_length(vec![5, 5, 5, 5, 5, 5, 5], 4),
            4
        );
    }
}
