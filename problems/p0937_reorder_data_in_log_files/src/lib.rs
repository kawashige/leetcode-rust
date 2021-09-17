pub struct Solution {}

impl Solution {
    pub fn reorder_log_files(mut logs: Vec<String>) -> Vec<String> {
        logs.sort_by_key(|log| {
            let mut sp = log.split(" ");
            let identifier = sp.next().unwrap().to_string();
            let contents = sp.collect::<Vec<&str>>().join(" ");
            if contents.as_bytes()[0].is_ascii_alphabetic() {
                (0, contents, identifier)
            } else {
                (1, "".to_string(), "".to_string())
            }
        });
        logs
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_0937() {
        assert_eq!(
            Solution::reorder_log_files(vec![
                "dig1 8 1 5 1".to_string(),
                "let1 art can".to_string(),
                "dig2 3 6".to_string(),
                "let2 own kit dig".to_string(),
                "let3 art zero".to_string()
            ]),
            vec![
                "let1 art can".to_string(),
                "let3 art zero".to_string(),
                "let2 own kit dig".to_string(),
                "dig1 8 1 5 1".to_string(),
                "dig2 3 6".to_string()
            ]
        );
        assert_eq!(
            Solution::reorder_log_files(vec![
                "a1 9 2 3 1".to_string(),
                "g1 act car".to_string(),
                "zo4 4 7".to_string(),
                "ab1 off key dog".to_string(),
                "a8 act zoo".to_string()
            ]),
            vec![
                "g1 act car".to_string(),
                "a8 act zoo".to_string(),
                "ab1 off key dog".to_string(),
                "a1 9 2 3 1".to_string(),
                "zo4 4 7".to_string()
            ]
        );
    }
}
