pub struct Solution {}

impl Solution {
    pub fn calc(d2: i32, num_stack: &mut Vec<Vec<i32>>, op_stack: &mut Vec<Vec<char>>) {
        match op_stack.last().unwrap().last() {
            Some('+') => {
                let d1 = num_stack.last_mut().unwrap().pop().unwrap();
                num_stack.last_mut().unwrap().push(d1 + d2);
                op_stack.last_mut().unwrap().pop();
            }
            Some('-') => {
                if let Some(d1) = num_stack.last_mut().unwrap().pop() {
                    num_stack.last_mut().unwrap().push(d1 - d2);
                } else {
                    num_stack.last_mut().unwrap().push(-d2);
                }
                op_stack.last_mut().unwrap().pop();
            }
            None => {
                num_stack.last_mut().unwrap().push(d2);
            }
            _ => unreachable!(),
        }
    }

    pub fn calculate(s: String) -> i32 {
        let mut num_stack = vec![vec![]];
        let mut op_stack = vec![vec![]];
        let mut d = None;

        for c in s.chars() {
            match c {
                '(' => {
                    num_stack.push(vec![]);
                    op_stack.push(vec![]);
                }
                ')' => {
                    if let Some(x) = d {
                        Self::calc(x, &mut num_stack, &mut op_stack);
                        d = None;
                    }
                    let v = num_stack.pop().unwrap();
                    op_stack.pop().unwrap();
                    Self::calc(v[0], &mut num_stack, &mut op_stack);
                }
                '+' | '-' => {
                    if let Some(x) = d {
                        Self::calc(x, &mut num_stack, &mut op_stack);
                        d = None;
                    }
                    (*op_stack.last_mut().unwrap()).push(c);
                }
                ' ' => {}
                _ => {
                    d = Some(d.unwrap_or(0) * 10 + c.to_digit(10).unwrap() as i32);
                }
            }
        }
        if let Some(x) = d {
            Self::calc(x, &mut num_stack, &mut op_stack);
        }

        *num_stack.last().unwrap().last().unwrap()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_day11() {
        assert_eq!(Solution::calculate("(7)-(0)+(4)".to_string()), 11);
        assert_eq!(Solution::calculate("1 + 1".to_string()), 2);
        assert_eq!(Solution::calculate("2-1 + 2 ".to_string()), 3);
        assert_eq!(Solution::calculate("(1+(4+5+2)-3)+(6+8)".to_string()), 23);
    }
}
