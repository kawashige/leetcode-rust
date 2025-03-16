use std::collections::HashMap;

pub struct Solution {}

impl Solution {
    pub fn maximum_subarray_sum(nums: Vec<i32>, k: i32) -> i64 {
        let mut map = HashMap::new();
        let mut sum = 0;
        let mut result = std::i64::MIN;

        for i in 0..nums.len() {
            let tmp = *map.get(&nums[i]).unwrap_or(&std::i64::MAX);
            map.insert(nums[i], sum.min(tmp));
            sum += nums[i] as i64;
            for t in [nums[i] - k, nums[i] + k] {
                if let Some(s) = map.get(&t) {
                    result = result.max(sum - s);
                }
            }
        }

        if result == std::i64::MIN {
            0
        } else {
            result
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_3026() {
        assert_eq!(
            Solution::maximum_subarray_sum(vec![1, 2, 3, 4, 5, 6], 1),
            11
        );
        assert_eq!(Solution::maximum_subarray_sum(vec![-1, 3, 2, 4, 5], 3), 11);
        assert_eq!(Solution::maximum_subarray_sum(vec![-1, -2, -3, -4], 2), -6);
    }
}
