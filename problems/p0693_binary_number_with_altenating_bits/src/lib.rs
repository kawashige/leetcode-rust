pub struct Solution {}

impl Solution {
    pub fn has_alternating_bits(n: i32) -> bool {
        let s = format!("{:b}", n);
        !s.contains("00") && !s.contains("11")
    }
}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn test_0693() {
        assert!(!Solution::has_alternating_bits(4));
        assert!(Solution::has_alternating_bits(5));
        assert!(!Solution::has_alternating_bits(7));
        assert!(!Solution::has_alternating_bits(11));
        assert!(Solution::has_alternating_bits(10));
        assert!(!Solution::has_alternating_bits(3));
    }
}
