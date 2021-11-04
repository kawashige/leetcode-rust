pub struct Solution {}

impl Solution {
    pub fn is_valid(s: String) -> bool {
        let mut stack = Vec::with_capacity(s.len());
        for b in s.as_bytes() {
            if b == &b'c'
                && stack.len() > 1
                && stack[stack.len() - 2] == &b'a'
                && stack[stack.len() - 1] == &b'b'
            {
                stack.pop();
                stack.pop();
            } else {
                stack.push(b);
            }
        }

        stack.is_empty()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_1003() {
        assert!(Solution::is_valid("aabcbc".to_string()));
        assert!(Solution::is_valid("abcabcababcc".to_string()));
        assert!(!Solution::is_valid("abccba".to_string()));
        assert!(!Solution::is_valid("cababc".to_string()));
    }
}
