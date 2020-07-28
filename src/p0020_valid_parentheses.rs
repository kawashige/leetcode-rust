pub struct Solution {}

impl Solution {
    pub fn is_valid(s: String) -> bool {
        let open = "({[";
        let mut stack = Vec::new();
        for x in s.chars() {
            if open.contains(x) {
                stack.push(x.clone());
            } else {
                match stack.pop() {
                    Some(y) => match (y, x) {
                        ('(', ')') | ('{', '}') | ('[', ']') => continue,
                        _ => return false,
                    },
                    _ => return false,
                }
            }
        }
        if stack.len() == 0 {
            true
        } else {
            false
        }
    }
}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn test_20() {
        assert!(!Solution::is_valid("(){}}".to_string()));
        assert!(Solution::is_valid("(){}".to_string()));
    }
}
