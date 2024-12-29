use std::collections::HashMap;

pub struct Solution {}

impl Solution {
    pub fn min_size_subarray(nums: Vec<i32>, target: i32) -> i32 {
        let sum = nums.iter().sum::<i32>();
        let result = (target / sum) * nums.len() as i32;
        let target = target - sum * (target / sum);
        if target == 0 {
            return result;
        }

        let mut sum = 0;
        let mut map = HashMap::new();
        map.insert(0, 0);
        let mut len = std::i32::MAX;
        for i in 0..nums.len() * 2 {
            sum += nums[i % nums.len()];
            if target <= sum {
                if let Some(j) = map.get(&(sum - target)) {
                    len = len.min((i + 1 - j) as i32);
                }
            }
            map.insert(sum, i + 1);
        }

        if len == std::i32::MAX {
            -1
        } else {
            result + len
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_2875() {
        assert_eq!(
            Solution::min_size_subarray(vec![1, 6, 5, 5, 1, 1, 2, 5, 3, 1, 5, 3, 2, 4, 6, 6], 56),
            16
        );
        assert_eq!(Solution::min_size_subarray(vec![1, 2, 3], 5), 2);
        assert_eq!(Solution::min_size_subarray(vec![1, 1, 1, 2, 3], 4), 2);
        assert_eq!(Solution::min_size_subarray(vec![2, 4, 6, 8], 3), -1);
    }
}
