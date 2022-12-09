use std::collections::HashMap;

pub struct Solution {}

impl Solution {
    pub fn evaluate(s: String, knowledge: Vec<Vec<String>>) -> String {
        let knowledge = knowledge
            .into_iter()
            .map(|k| (k[0].clone(), k[1].clone()))
            .collect::<HashMap<_, _>>();

        let mut result = String::new();
        let mut i = 0;
        while i < s.len() {
            if s.as_bytes()[i] == b'(' {
                if let Some(j) = (i + 1..s.len()).find(|j| s.as_bytes()[*j] == b')') {
                    result += knowledge.get(&s[i + 1..j]).unwrap_or(&"?".to_string());
                    i = j + 1;
                }
            } else {
                result.push(s.as_bytes()[i] as char);
                i += 1;
            }
        }
        result
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_1807() {
        assert_eq!(
            Solution::evaluate(
                "(name)is(age)yearsold".to_string(),
                vec![
                    vec!["name".to_string(), "bob".to_string()],
                    vec!["age".to_string(), "two".to_string()]
                ]
            ),
            "bobistwoyearsold".to_string()
        );
        assert_eq!(
            Solution::evaluate(
                "hi(name)".to_string(),
                vec![vec!["a".to_string(), "b".to_string()]]
            ),
            "hi?".to_string()
        );
        assert_eq!(
            Solution::evaluate(
                "(a)(a)(a)aaa".to_string(),
                vec![vec!["a".to_string(), "yes".to_string()]]
            ),
            "yesyesyesaaa".to_string()
        );
    }
}
