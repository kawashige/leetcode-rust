pub struct Solution {}

impl Solution {
    pub fn min_operations_to_make_median_k(nums: Vec<i32>, k: i32) -> i64 {
        let mut nums = nums;
        nums.sort_unstable();
        (0..nums.len()).fold(0, |sum, i| {
            if i == nums.len() / 2 {
                sum + ((nums[i] - k) as i64).abs()
            } else if i <= nums.len() / 2 {
                sum + ((nums[i] - k) as i64).max(0)
            } else {
                sum + ((k - nums[i]) as i64).max(0)
            }
        })
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_3107() {
        assert_eq!(
            Solution::min_operations_to_make_median_k(vec![2, 5, 6, 8, 5], 4),
            2
        );
        assert_eq!(
            Solution::min_operations_to_make_median_k(vec![2, 5, 6, 8, 5], 7),
            3
        );
        assert_eq!(
            Solution::min_operations_to_make_median_k(vec![1, 2, 3, 4, 5, 6], 4),
            0
        );
    }
}
