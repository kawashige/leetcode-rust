pub struct Solution {}

impl Solution {
    pub fn generate_parenthesis(n: i32) -> Vec<String> {
        pub fn generate(s: String, o: i32, c: i32) -> Vec<String> {
            let mut result = Vec::new();
            if o == 0 && c == 0 {
                result.push(s);
            } else {
                if o > 0 {
                    result.append(&mut generate(format!("{}{}", s, "("), o - 1, c));
                }
                if o < c {
                    result.append(&mut generate(format!("{}{}", s, ")"), o, c - 1));
                }
            }
            result
        }

        generate("".to_string(), n, n)
    }
}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn test_0022() {
        let result = vec![
            "((()))".to_string(),
            "(()())".to_string(),
            "(())()".to_string(),
            "()(())".to_string(),
            "()()()".to_string(),
        ];
        assert_eq!(result, Solution::generate_parenthesis(3));
        assert_eq!(vec!["()".to_string()], Solution::generate_parenthesis(1));
    }
}
