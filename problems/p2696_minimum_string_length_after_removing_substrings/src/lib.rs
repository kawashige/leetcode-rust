pub struct Solution {}

impl Solution {
    pub fn min_length(s: String) -> i32 {
        let mut stack = Vec::with_capacity(s.len());
        for i in 0..s.len() {
            if !stack.is_empty()
                && ((stack.last() == Some(&b'A') && s.as_bytes()[i] == b'B')
                    || (stack.last() == Some(&b'C') && s.as_bytes()[i] == b'D'))
            {
                stack.pop();
            } else {
                stack.push(s.as_bytes()[i]);
            }
        }
        stack.len() as i32
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_2696() {
        assert_eq!(Solution::min_length("ABFCACDB".to_string()), 2);
        assert_eq!(Solution::min_length("ACBBD".to_string()), 5);
    }
}
