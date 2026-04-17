use std::collections::HashMap;

pub struct Solution {}

impl Solution {
    pub fn min_mirror_pair_distance(nums: Vec<i32>) -> i32 {
        let mut map = HashMap::new();
        let mut result = std::usize::MAX;

        for i in 0..nums.len() {
            if let Some(j) = map.get(&nums[i]) {
                result = result.min(i - j);
            }
            let mut num = nums[i];
            while num % 10 == 0 {
                num /= 10;
            }
            let mut reversed_num = 0;
            while 0 < num {
                reversed_num = reversed_num * 10 + num % 10;
                num /= 10;
            }
            map.insert(reversed_num, i);
        }

        if result == std::usize::MAX {
            -1
        } else {
            result as i32
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_3761() {
        assert_eq!(
            Solution::min_mirror_pair_distance(vec![12, 21, 45, 33, 54]),
            1
        );
        assert_eq!(Solution::min_mirror_pair_distance(vec![120, 21]), 1);
        assert_eq!(Solution::min_mirror_pair_distance(vec![21, 120]), -1);
    }
}
