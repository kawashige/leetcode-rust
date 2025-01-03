use std::collections::HashSet;

pub struct Solution {}

impl Solution {
    pub fn remove_invalid_parentheses(s: String) -> Vec<String> {
        let mut result = Vec::new();
        let c = s
            .as_bytes()
            .iter()
            .filter(|b| b == &&b'(' || b == &&b')')
            .count();
        for i in 0..2_usize.pow(c as u32) {
            let mut p = 0;
            let mut new_s = String::new();
            let mut count = 0;
            for j in 0..s.len() {
                if s.as_bytes()[j] == b'(' || s.as_bytes()[j] == b')' {
                    if i & 1 << p == 0 {
                        if s.as_bytes()[j] == b'(' {
                            count += 1;
                        } else {
                            count -= 1;
                            if count < 0 {
                                break;
                            }
                        }
                        new_s.push(s.as_bytes()[j] as char);
                    }
                    p += 1;
                } else {
                    new_s.push(s.as_bytes()[j] as char);
                }
            }
            if count == 0 && new_s.len() + i.count_ones() as usize == s.len() {
                result.push(new_s);
            }
        }
        if result.is_empty() {
            result
        } else {
            let max = result.iter().map(|r| r.len()).max().unwrap();
            result
                .into_iter()
                .filter(|r| r.len() == max)
                .collect::<HashSet<_>>()
                .into_iter()
                .collect()
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_0301() {
        assert_eq!(
            Solution::remove_invalid_parentheses("()())()".to_string()),
            vec!["(())()".to_string(), "()()()".to_string()]
        );
        assert_eq!(
            Solution::remove_invalid_parentheses("(a)())()".to_string()),
            vec!["(a())()".to_string(), "(a)()()".to_string()]
        );
        assert_eq!(
            Solution::remove_invalid_parentheses(")(".to_string()),
            vec!["".to_string()]
        );
    }
}
