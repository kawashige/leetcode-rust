pub struct Solution {}

impl Solution {
    pub fn cal_points(ops: Vec<String>) -> i32 {
        let mut stack = Vec::with_capacity(ops.len());
        for op in ops {
            match op.as_str() {
                "C" => {
                    stack.pop();
                }
                "D" => {
                    stack.push(stack.last().unwrap() * 2);
                }
                "+" => {
                    let l = stack.len() - 1;
                    stack.push(stack[l] + stack[l - 1])
                }
                _ => stack.push(op.parse::<i32>().unwrap()),
            }
        }
        stack.into_iter().sum()
    }
}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn test_0682() {
        assert_eq!(
            Solution::cal_points(vec![
                "5".to_string(),
                "2".to_string(),
                "C".to_string(),
                "D".to_string(),
                "+".to_string()
            ]),
            30
        );
        assert_eq!(
            Solution::cal_points(vec![
                "5".to_string(),
                "-2".to_string(),
                "4".to_string(),
                "C".to_string(),
                "D".to_string(),
                "9".to_string(),
                "+".to_string(),
                "+".to_string()
            ]),
            27
        );
        assert_eq!(Solution::cal_points(vec!["1".to_string()]), 1);
    }
}
