#[derive(Debug, PartialEq, Eq)]
pub enum NestedInteger {
    Int(i32),
    List(Vec<NestedInteger>),
}

pub struct Solution {}

impl Solution {
    pub fn deserialize(s: String) -> NestedInteger {
        let mut stack = Vec::new();
        let mut nums = Vec::new();
        for c in s.chars() {
            if c.is_numeric() || c == '-' {
                nums.push(c);
            } else if c == '[' {
                stack.push(Vec::new());
            } else if c == ']' {
                if !nums.is_empty() {
                    let num = nums.iter().collect::<String>().parse::<i32>().unwrap();
                    stack.last_mut().unwrap().push(NestedInteger::Int(num));
                    nums.clear();
                }
                if 1 < stack.len() {
                    let last = stack.pop().unwrap();
                    stack.last_mut().unwrap().push(NestedInteger::List(last))
                }
            } else if c == ',' {
                if !nums.is_empty() {
                    let num = nums.iter().collect::<String>().parse::<i32>().unwrap();
                    stack.last_mut().unwrap().push(NestedInteger::Int(num));
                    nums.clear();
                }
            }
        }

        if !nums.is_empty() && stack.is_empty() {
            let num = nums.into_iter().collect::<String>().parse::<i32>().unwrap();
            NestedInteger::Int(num)
        } else {
            NestedInteger::List(stack.pop().unwrap())
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_0385() {
        assert_eq!(
            NestedInteger::Int(0),
            Solution::deserialize("0".to_string())
        );
        assert_eq!(
            NestedInteger::Int(324),
            Solution::deserialize("324".to_string())
        );
        assert_eq!(
            NestedInteger::List(vec![NestedInteger::Int(324)]),
            Solution::deserialize("[324]".to_string())
        );
        assert_eq!(
            NestedInteger::List(vec![
                NestedInteger::Int(123),
                NestedInteger::List(vec![
                    NestedInteger::Int(456),
                    NestedInteger::List(vec![NestedInteger::Int(789)])
                ])
            ]),
            Solution::deserialize("[123,[456,[789]]]".to_string())
        );
        assert_eq!(
            NestedInteger::List(vec![
                NestedInteger::Int(123),
                NestedInteger::List(vec![
                    NestedInteger::List(vec![NestedInteger::Int(789)]),
                    NestedInteger::Int(-456),
                ])
            ]),
            Solution::deserialize("[123,[[789],-456]]".to_string())
        );
    }
}
