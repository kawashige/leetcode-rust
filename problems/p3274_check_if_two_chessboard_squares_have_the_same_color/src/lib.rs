pub struct Solution {}

impl Solution {
    pub fn check_two_chessboards(coordinate1: String, coordinate2: String) -> bool {
        if (coordinate1.as_bytes()[0] - b'a') % 2 == (coordinate2.as_bytes()[0] - b'a') % 2 {
            (coordinate1.as_bytes()[1] - b'0') % 2 == (coordinate2.as_bytes()[1] - b'0') % 2
        } else {
            (coordinate1.as_bytes()[1] - b'0') % 2 != (coordinate2.as_bytes()[1] - b'0') % 2
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_3274() {
        assert!(Solution::check_two_chessboards(
            "a1".to_string(),
            "c3".to_string()
        ));
        assert!(!Solution::check_two_chessboards(
            "a1".to_string(),
            "h3".to_string()
        ));
    }
}
