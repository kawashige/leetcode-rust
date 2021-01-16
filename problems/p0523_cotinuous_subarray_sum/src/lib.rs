pub struct Solution {}

use std::collections::HashMap;
impl Solution {
    pub fn check_subarray_sum(nums: Vec<i32>, k: i32) -> bool {
        let mut rem_map = HashMap::new();
        rem_map.insert(0, -1_i32);

        let mut rem = 0;
        for (i, n) in nums.into_iter().enumerate() {
            rem += n;
            rem = rem.checked_rem(k).unwrap_or(rem);
            if let Some(s) = rem_map.get(&rem) {
                if 2 <= i as i32 - s {
                    return true;
                }
            } else {
                rem_map.insert(rem, i as i32);
            }
        }

        false
    }
}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn test_0523() {
        assert!(Solution::check_subarray_sum(vec![0, 0], 0));
        assert!(!Solution::check_subarray_sum(vec![23], 1));
        assert!(Solution::check_subarray_sum(vec![23, 2, 4, 6, 7], 1));
        assert!(!Solution::check_subarray_sum(vec![23, 2, 4, 6, 7], 0));
        assert!(Solution::check_subarray_sum(vec![23, 2, 4, 6, 7], 6));
        assert!(Solution::check_subarray_sum(vec![23, 2, 6, 4, 7], 6));
        assert!(!Solution::check_subarray_sum(vec![1, 2, 3, 4, 5], 100));
    }
}
