pub struct Solution {}

impl Solution {
    pub fn k_length_apart(nums: Vec<i32>, k: i32) -> bool {
        (0..nums.len())
            .try_fold(-1_i32, |last_index, i| match nums[i] {
                1 if -1 < last_index && i as i32 - last_index - 1 < k => Err(()),
                1 => Ok(i as i32),
                _ => Ok(last_index),
            })
            .is_ok()
    }
}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn test_day25() {
        assert!(!Solution::k_length_apart(vec![1, 0, 1], 2));
        assert!(Solution::k_length_apart(vec![1, 0, 0, 0, 1, 0, 0, 1], 2));
        assert!(!Solution::k_length_apart(vec![1, 0, 0, 1, 0, 1], 2));
        assert!(Solution::k_length_apart(vec![1, 1, 1, 1, 1], 0));
        assert!(Solution::k_length_apart(vec![0, 1, 0, 1], 1));
    }
}
