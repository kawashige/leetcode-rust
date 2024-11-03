use std::collections::HashMap;

pub struct Solution {}

impl Solution {
    pub fn minimum_seconds(nums: Vec<i32>) -> i32 {
        let mut map = HashMap::new();
        for i in 0..nums.len() {
            (*map.entry(nums[i]).or_insert(vec![])).push(i);
        }

        let mut result = std::i32::MAX;

        for v in map.values() {
            if v.len() == 1 {
                result = result.min(nums.len() as i32 / 2);
            } else {
                let mut time = (v[0] + nums.len() - v[v.len() - 1]) / 2;
                for i in 0..v.len() - 1 {
                    time = time.max((v[i + 1] - v[i]) / 2);
                }
                result = result.min(time as i32);
            }
        }

        result
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_2808() {
        assert_eq!(Solution::minimum_seconds(vec![1, 2, 1, 2]), 1);
        assert_eq!(Solution::minimum_seconds(vec![2, 1, 3, 3, 2]), 2);
        assert_eq!(Solution::minimum_seconds(vec![5, 5, 5, 5]), 0);
    }
}
