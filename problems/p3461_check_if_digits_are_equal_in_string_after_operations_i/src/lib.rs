pub struct Solution {}

impl Solution {
    pub fn has_same_digits(s: String) -> bool {
        let mut digits = s
            .as_bytes()
            .iter()
            .map(|b| (b - b'0') as usize)
            .collect::<Vec<_>>();
        while 2 < digits.len() {
            digits = digits.windows(2).map(|x| (x[0] + x[1]) % 10).collect()
        }
        digits[0] == digits[1]
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_3461() {
        assert!(Solution::has_same_digits("3902".to_string()));
        assert!(!Solution::has_same_digits("34789".to_string()));
    }
}
