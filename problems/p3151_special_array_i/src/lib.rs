pub struct Solution {}

impl Solution {
    pub fn is_array_special(nums: Vec<i32>) -> bool {
        (1..nums.len()).all(|i| nums[i] % 2 != nums[i - 1] % 2)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_3151() {
        assert!(Solution::is_array_special(vec![1]));
        assert!(Solution::is_array_special(vec![2, 1, 4]));
        assert!(!Solution::is_array_special(vec![4, 3, 1, 6]));
    }
}
