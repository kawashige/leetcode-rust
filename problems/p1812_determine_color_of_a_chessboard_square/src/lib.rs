pub struct Solution {}

impl Solution {
    pub fn square_is_white(coordinates: String) -> bool {
        (coordinates.as_bytes()[0] - b'a') % 2 == (coordinates.as_bytes()[1] - b'0') % 2
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_1812() {
        assert!(!Solution::square_is_white("a1".to_string()));
        assert!(Solution::square_is_white("h3".to_string()));
        assert!(!Solution::square_is_white("c7".to_string()));
    }
}
