use std::ops::Index;

pub struct Solution {}

impl Solution {
    pub fn remove_occurrences(s: String, part: String) -> String {
        let mut stack = Vec::with_capacity(s.len());

        for i in 0..s.len() {
            stack.push(s.as_bytes()[i]);
            if part.len() <= stack.len()
                && (0..part.len())
                    .all(|j| stack[stack.len() - 1 - j] == part.as_bytes()[part.len() - 1 - j])
            {
                stack.truncate(stack.len() - part.len());
            }
        }

        String::from_utf8(stack).unwrap()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_1910() {
        assert_eq!(
            Solution::remove_occurrences("a".to_string(), "aa".to_string()),
            "a".to_string()
        );
        assert_eq!(
            Solution::remove_occurrences("daabcbaabcbc".to_string(), "abc".to_string()),
            "dab".to_string()
        );
        assert_eq!(
            Solution::remove_occurrences("axxxxyyyyb".to_string(), "xy".to_string()),
            "ab".to_string()
        );
    }
}
