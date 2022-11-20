pub struct Solution {}

impl Solution {
    pub fn check_ones_segment(s: String) -> bool {
        !(1..s.len()).any(|i| s.as_bytes()[i - 1] == b'0' && s.as_bytes()[i] == b'1')
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_1784() {
        assert!(!Solution::check_ones_segment("1001".to_string()));
        assert!(Solution::check_ones_segment("10".to_string()));
    }
}
