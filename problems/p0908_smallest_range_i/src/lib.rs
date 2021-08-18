pub struct Solution {}

impl Solution {
    pub fn smallest_range_i(mut nums: Vec<i32>, k: i32) -> i32 {
        nums.sort_unstable();
        (nums.last().unwrap() - k - (nums[0] + k)).max(0)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_0908() {
        assert_eq!(Solution::smallest_range_i(vec![1], 0), 0);
        assert_eq!(Solution::smallest_range_i(vec![0, 10], 2), 6);
        assert_eq!(Solution::smallest_range_i(vec![1, 3, 6], 3), 0);
    }
}
