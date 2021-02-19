pub struct Solution {}

impl Solution {
    pub fn min_remove_to_make_valid(s: String) -> String {
        let (mut chars, stack, delete) = s.chars().enumerate().fold(
            (Vec::new(), Vec::new(), Vec::new()),
            |(mut chars, mut delete, mut stack), (i, c)| {
                chars.push(c);
                match c {
                    '(' => stack.push(i),
                    ')' if stack.is_empty() => delete.push(i),
                    ')' => {
                        stack.pop();
                    }
                    _ => {}
                }
                (chars, delete, stack)
            },
        );

        for i in delete.into_iter().chain(stack.into_iter()) {
            chars[i] = ' ';
        }
        chars.into_iter().filter(|c| !c.is_whitespace()).collect()
    }
}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn test_day19() {
        assert_eq!(
            Solution::min_remove_to_make_valid("())()(((".to_string()),
            "()()".to_string()
        );
        assert_eq!(
            Solution::min_remove_to_make_valid("lee(t(c)o)de)".to_string()),
            "lee(t(c)o)de".to_string()
        );
        assert_eq!(
            Solution::min_remove_to_make_valid("a)b(c)d".to_string()),
            "ab(c)d".to_string()
        );
        assert_eq!(
            Solution::min_remove_to_make_valid("))((".to_string()),
            "".to_string()
        );
        assert_eq!(
            Solution::min_remove_to_make_valid("(a(b(c)d)".to_string()),
            "a(b(c)d)".to_string()
        );
    }
}
