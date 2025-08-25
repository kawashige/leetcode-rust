pub struct Solution {}

impl Solution {
    pub fn is_balanced(num: String) -> bool {
        num.as_bytes().iter().enumerate().fold(0, |acc, (i, b)| {
            acc + if i % 2 == 0 { 1 } else { -1 } * (b - b'0') as i32
        }) == 0
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_3340() {
        assert!(!Solution::is_balanced("1234".to_string()));
        assert!(Solution::is_balanced("24123".to_string()));
    }
}
