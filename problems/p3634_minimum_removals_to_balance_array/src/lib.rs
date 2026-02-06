pub struct Solution {}

impl Solution {
    pub fn min_removal(nums: Vec<i32>, k: i32) -> i32 {
        let mut nums = nums;
        nums.sort_unstable();

        let mut result = nums.len() - 1;
        let mut j = 0;

        for i in 0..nums.len() {
            if 0 < i && nums[i - 1] == nums[i] {
                continue;
            }
            j = j.max(i);
            while j + 1 < nums.len() && nums[j + 1] as i64 <= nums[i] as i64 * k as i64 {
                j += 1;
            }
            result = result.min(nums.len() - (j - i + 1));
        }

        result as i32
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_3634() {
        assert_eq!(Solution::min_removal(vec![2, 1, 5], 2), 1);
        assert_eq!(Solution::min_removal(vec![1, 6, 2, 9], 3), 2);
        assert_eq!(Solution::min_removal(vec![4, 6], 2), 0);
    }
}
