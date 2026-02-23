pub struct Solution {}

impl Solution {
    pub fn resulting_string(s: String) -> String {
        let mut stack = vec![s.as_bytes()[0] as char];
        for i in 1..s.len() {
            if !stack.is_empty()
                && (stack.last().unwrap() == &(((s.as_bytes()[i] - b'a' + 1) % 26 + b'a') as char)
                    || stack.last().unwrap()
                        == &(((s.as_bytes()[i] - b'a' + 25) % 26 + b'a') as char))
            {
                stack.pop().unwrap();
            } else {
                stack.push(s.as_bytes()[i] as char);
            }
        }
        stack.into_iter().collect()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_3561() {
        assert_eq!(
            Solution::resulting_string("abc".to_string()),
            "c".to_string()
        );
        assert_eq!(
            Solution::resulting_string("adcb".to_string()),
            "".to_string()
        );
        assert_eq!(
            Solution::resulting_string("zadb".to_string()),
            "db".to_string()
        );
    }
}
