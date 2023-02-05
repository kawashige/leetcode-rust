pub struct Solution {}

impl Solution {
    pub fn count_good_substrings(s: String) -> i32 {
        s.as_bytes()
            .windows(3)
            .filter(|bytes| bytes[0] != bytes[1] && bytes[1] != bytes[2] && bytes[2] != bytes[0])
            .count() as i32
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_1876() {
        assert_eq!(Solution::count_good_substrings("x".to_string()), 0);
        assert_eq!(Solution::count_good_substrings("xyzzaz".to_string()), 1);
        assert_eq!(Solution::count_good_substrings("aababcabc".to_string()), 4);
    }
}
