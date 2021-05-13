pub struct Solution {}

impl Solution {
    pub fn candidates(s: &str) -> Vec<String> {
        (1..=s.len())
            .filter_map(|i| {
                if (i == 1 || !s.starts_with("0")) && (i == s.len() || !s.ends_with("0")) {
                    Some(if i == s.len() {
                        s.to_string()
                    } else {
                        format!("{}.{}", &s[..i], &s[i..])
                    })
                } else {
                    None
                }
            })
            .collect()
    }

    pub fn ambiguous_coordinates(s: String) -> Vec<String> {
        let mut result = Vec::new();
        for i in 1..(s.len() - 1) {
            for x in Self::candidates(&s[1..i]) {
                for y in Self::candidates(&s[i..(s.len() - 1)]) {
                    result.push(format!("({}, {})", x, y));
                }
            }
        }
        result
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_day13() {
        assert_eq!(
            Solution::ambiguous_coordinates("(123)".to_string()),
            vec![
                "(1, 2.3)".to_string(),
                "(1, 23)".to_string(),
                "(1.2, 3)".to_string(),
                "(12, 3)".to_string(),
            ]
        );

        assert_eq!(
            Solution::ambiguous_coordinates("(00011)".to_string()),
            vec!["(0, 0.011)".to_string(), "(0.001, 1)".to_string(),]
        );

        assert_eq!(
            Solution::ambiguous_coordinates("(0123)".to_string()),
            vec![
                "(0, 1.23)".to_string(),
                "(0, 12.3)".to_string(),
                "(0, 123)".to_string(),
                "(0.1, 2.3)".to_string(),
                "(0.1, 23)".to_string(),
                "(0.12, 3)".to_string(),
            ]
        );

        assert_eq!(
            Solution::ambiguous_coordinates("(100)".to_string()),
            vec!["(10, 0)".to_string()]
        );
    }
}
