pub struct Solution {}

impl Solution {
    pub fn has_trailing_zeros(nums: Vec<i32>) -> bool {
        1 < nums.into_iter().filter(|n| 0 < n.trailing_zeros()).count()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_2980() {
        assert!(Solution::has_trailing_zeros(vec![1, 2, 3, 4, 5]));
        assert!(Solution::has_trailing_zeros(vec![2, 4, 8, 16]));
        assert!(!Solution::has_trailing_zeros(vec![1, 3, 5, 7, 9]));
    }
}
