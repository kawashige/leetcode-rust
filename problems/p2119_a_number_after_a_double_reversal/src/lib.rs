pub struct Solution {}

impl Solution {
    pub fn is_same_after_reversals(num: i32) -> bool {
        num == 0 || !num.to_string().ends_with('0')
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_2119() {
        assert!(Solution::is_same_after_reversals(526));
        assert!(!Solution::is_same_after_reversals(1000));
        assert!(Solution::is_same_after_reversals(0));
    }
}
