pub struct Solution {}

impl Solution {
    pub fn can_split_array(nums: Vec<i32>, m: i32) -> bool {
        if nums.len() < 3 {
            return true;
        }
        (1..nums.len()).any(|i| m <= nums[i - 1] + nums[i])
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_2811() {
        assert!(Solution::can_split_array(vec![2, 2, 1], 4));
        assert!(!Solution::can_split_array(vec![2, 1, 3], 5));
        assert!(Solution::can_split_array(vec![2, 3, 3, 2, 3], 6));
    }
}
