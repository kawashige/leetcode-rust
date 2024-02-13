use std::collections::HashSet;

pub struct Solution {}

impl Solution {
    pub fn find_subarrays(nums: Vec<i32>) -> bool {
        let mut set = HashSet::new();

        for v in nums.windows(2) {
            if !set.insert(v[0] + v[1]) {
                return true;
            }
        }

        false
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_2395() {
        assert!(Solution::find_subarrays(vec![4, 2, 4]));
        assert!(Solution::find_subarrays(vec![0, 0, 0]));
    }
}
