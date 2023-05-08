pub struct Solution {}

impl Solution {
    pub fn minimum_difference(nums: Vec<i32>, k: i32) -> i32 {
        let mut nums = nums;
        nums.sort_unstable();
        (0..nums.len() - k as usize + 1)
            .map(|i| nums[i + k as usize - 1] - nums[i])
            .min()
            .unwrap_or(0)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_1984() {
        assert_eq!(Solution::minimum_difference(vec![90], 1), 0);
        assert_eq!(Solution::minimum_difference(vec![9, 4, 1, 7], 2), 2);
    }
}
