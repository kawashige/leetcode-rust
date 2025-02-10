pub struct Solution {}

impl Solution {
    pub fn clear_digits(s: String) -> String {
        let mut stack = Vec::new();
        for c in s.chars() {
            if c.is_digit(10) {
                stack.pop();
            } else {
                stack.push(c);
            }
        }
        stack.into_iter().collect()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_3174() {
        assert_eq!(Solution::clear_digits("abc".to_string()), "abc".to_string());
        assert_eq!(Solution::clear_digits("cb34".to_string()), "".to_string());
    }
}
