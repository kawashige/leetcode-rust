pub struct Solution {}

impl Solution {
    pub fn longest_equal_subarray(nums: Vec<i32>, k: i32) -> i32 {
        let mut nums = nums.into_iter().zip(0..).collect::<Vec<_>>();
        nums.sort_unstable();

        let mut result = 0;

        let mut l = 0;
        let mut r = 0;

        while l < nums.len() {
            let mut deleted = 0;
            while r + 1 < nums.len() && nums[r + 1].0 == nums[r].0 {
                r += 1;
                deleted += nums[r].1 - nums[r - 1].1 - 1;
                while k < deleted {
                    l += 1;
                    deleted -= nums[l].1 - nums[l - 1].1 - 1;
                }
                result = result.max(r - l + 1);
            }
            l = r + 1;
            r = l;
        }

        result as i32
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_2831() {
        assert_eq!(
            Solution::longest_equal_subarray(vec![1, 3, 2, 3, 1, 3], 3),
            3
        );
        assert_eq!(
            Solution::longest_equal_subarray(vec![1, 1, 2, 2, 1, 1], 2),
            4
        );
    }
}
