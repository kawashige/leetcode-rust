pub struct Solution {}

impl Solution {
    pub fn letter_case_permutation(s: String) -> Vec<String> {
        s.chars().fold(vec![String::new()], |strings, c| {
            strings
                .into_iter()
                .map(|mut s1| {
                    if c.is_alphabetic() {
                        let mut s2 = s1.clone();
                        s1.push(c.to_ascii_lowercase());
                        s2.push(c.to_ascii_uppercase());
                        vec![s1, s2]
                    } else {
                        s1.push(c);
                        vec![s1]
                    }
                })
                .flatten()
                .collect()
        })
    }
}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn test_day16() {
        assert_eq!(
            Solution::letter_case_permutation("a1b2".to_string()),
            vec![
                "a1b2".to_string(),
                "a1B2".to_string(),
                "A1b2".to_string(),
                "A1B2".to_string()
            ]
        );
        assert_eq!(
            Solution::letter_case_permutation("3z4".to_string()),
            vec!["3z4".to_string(), "3Z4".to_string()]
        );
        assert_eq!(
            Solution::letter_case_permutation("12345".to_string()),
            vec!["12345".to_string()]
        );
        assert_eq!(
            Solution::letter_case_permutation("0".to_string()),
            vec!["0".to_string()]
        );
    }
}
