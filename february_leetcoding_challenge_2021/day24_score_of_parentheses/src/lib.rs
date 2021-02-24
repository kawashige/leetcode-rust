pub struct Solution {}

impl Solution {
    pub fn score_of_parentheses(s: String) -> i32 {
        *s.chars()
            .fold(vec![0], |mut stack, c| {
                if c == '(' {
                    stack.push(0);
                } else {
                    let n = stack.pop().unwrap();
                    *stack.last_mut().unwrap() += if n == 0 { 1 } else { 2 * n };
                }
                stack
            })
            .first()
            .unwrap()
    }
}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn test_day24() {
        assert_eq!(Solution::score_of_parentheses("()".to_string()), 1);
        assert_eq!(Solution::score_of_parentheses("(())".to_string()), 2);
        assert_eq!(Solution::score_of_parentheses("()()".to_string()), 2);
        assert_eq!(Solution::score_of_parentheses("(()(()))".to_string()), 6);
    }
}
