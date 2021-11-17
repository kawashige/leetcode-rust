pub struct Solution {}

impl Solution {
    pub fn remove_outer_parentheses(s: String) -> String {
        let mut count = 0;
        let mut r = String::new();
        for c in s.chars() {
            if (c == '(' && count != 0) || (c == ')' && count != 1) {
                r.push(c);
            }
            count += if c == '(' { 1 } else { -1 };
        }
        r
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_1021() {
        assert_eq!(
            Solution::remove_outer_parentheses("(()())(())".to_string()),
            "()()()".to_string()
        );
        assert_eq!(
            Solution::remove_outer_parentheses("(()())(())(()(()))".to_string()),
            "()()()()(())".to_string()
        );
        assert_eq!(
            Solution::remove_outer_parentheses("()()".to_string()),
            "".to_string()
        );
    }
}
