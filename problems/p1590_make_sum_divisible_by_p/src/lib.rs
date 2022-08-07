use std::collections::HashMap;

pub struct Solution {}

impl Solution {
    pub fn min_subarray(nums: Vec<i32>, p: i32) -> i32 {
        let remains = nums.iter().fold(0, |acc, num| (acc + num) % p);

        if remains == 0 {
            return 0;
        }

        let mut min_len = nums.len() as i32;
        let mut map = HashMap::new();
        map.insert(0, -1);

        let mut tmp_remains = 0;
        for i in 0..nums.len() {
            tmp_remains = (tmp_remains + nums[i]) % p;
            let target = (p + tmp_remains - remains) % p;
            if let Some(j) = map.get(&target) {
                min_len = min_len.min(i as i32 - j);
            }
            map.insert(tmp_remains, i as i32);
        }

        if min_len == nums.len() as i32 {
            -1
        } else {
            min_len
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_1590() {
        assert_eq!(
            Solution::min_subarray(vec![1000000000, 1000000000, 1000000000], 3),
            0
        );
        assert_eq!(Solution::min_subarray(vec![3, 1, 4, 2], 6), 1);
        assert_eq!(Solution::min_subarray(vec![6, 3, 5, 2], 9), 2);
        assert_eq!(Solution::min_subarray(vec![1, 2, 3], 3), 0);
        assert_eq!(Solution::min_subarray(vec![3], 2), -1);
    }
}
