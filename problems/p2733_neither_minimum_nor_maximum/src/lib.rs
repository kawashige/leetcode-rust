pub struct Solution {}

impl Solution {
    pub fn find_non_min_or_max(nums: Vec<i32>) -> i32 {
        if nums.len() < 3 {
            return -1;
        }
        let mut nums = nums[..3].to_vec();
        nums.sort_unstable();
        nums[1]
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_2733() {
        assert_eq!(Solution::find_non_min_or_max(vec![3, 2, 1, 4]), 2);
        assert_eq!(Solution::find_non_min_or_max(vec![1, 2]), -1);
        assert_eq!(Solution::find_non_min_or_max(vec![2, 1, 3]), 2);
    }
}
