pub struct Solution {}

impl Solution {
    pub fn eval_rpn(tokens: Vec<String>) -> i32 {
        fn recurse(tokens: &mut Vec<String>) -> i32 {
            let operators = vec![
                "+".to_string(),
                "-".to_string(),
                "*".to_string(),
                "/".to_string(),
            ];

            let op = tokens.pop().unwrap();
            if !operators.contains(&op) {
                return op.parse::<i32>().unwrap();
            }
            let n2 = if operators.contains(tokens.last().unwrap()) {
                recurse(tokens)
            } else {
                tokens.pop().unwrap().parse::<i32>().unwrap()
            };
            let n1 = if operators.contains(tokens.last().unwrap()) {
                recurse(tokens)
            } else {
                tokens.pop().unwrap().parse::<i32>().unwrap()
            };

            match op.as_str() {
                "+" => n1 + n2,
                "-" => n1 - n2,
                "*" => n1 * n2,
                "/" => n1 / n2,
                _ => unreachable!(),
            }
        }

        let mut tokens = tokens;
        recurse(&mut tokens)
    }
}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn test_0150() {
        assert_eq!(
            9,
            Solution::eval_rpn(vec![
                "2".to_string(),
                "1".to_string(),
                "+".to_string(),
                "3".to_string(),
                "*".to_string()
            ])
        );
        assert_eq!(
            6,
            Solution::eval_rpn(vec![
                "4".to_string(),
                "13".to_string(),
                "5".to_string(),
                "/".to_string(),
                "+".to_string()
            ])
        );
        assert_eq!(
            22,
            Solution::eval_rpn(vec![
                "10".to_string(),
                "6".to_string(),
                "9".to_string(),
                "3".to_string(),
                "+".to_string(),
                "-11".to_string(),
                "*".to_string(),
                "/".to_string(),
                "*".to_string(),
                "17".to_string(),
                "+".to_string(),
                "5".to_string(),
                "+".to_string()
            ])
        );
        assert_eq!(18, Solution::eval_rpn(vec!["18".to_string()]));
    }
}
