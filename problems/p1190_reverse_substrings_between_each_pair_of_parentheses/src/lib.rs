pub struct Solution {}

impl Solution {
    pub fn reverse_parentheses(s: String) -> String {
        let mut stack = vec![vec![]];
        for c in s.chars() {
            match c {
                '(' => stack.push(vec![]),
                ')' => {
                    let chars = stack.pop().unwrap();
                    for c in chars.into_iter().rev() {
                        (*stack.last_mut().unwrap()).push(c);
                    }
                }
                _ => {
                    (*stack.last_mut().unwrap()).push(c);
                }
            }
        }

        stack[0].iter().collect()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_1190() {
        assert_eq!(
            Solution::reverse_parentheses("ta()usw((((a))))".to_string()),
            "tauswa".to_string()
        );
        assert_eq!(
            Solution::reverse_parentheses("abc(de)fg".to_string()),
            "abcedfg".to_string()
        );
        assert_eq!(
            Solution::reverse_parentheses("(abcd)".to_string()),
            "dcba".to_string()
        );
        assert_eq!(
            Solution::reverse_parentheses("(u(love)i)".to_string()),
            "iloveu".to_string()
        );
        assert_eq!(
            Solution::reverse_parentheses("(ed(et(oc))el)".to_string()),
            "leetcode".to_string()
        );
    }
}
