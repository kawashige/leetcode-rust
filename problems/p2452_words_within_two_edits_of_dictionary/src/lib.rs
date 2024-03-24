pub struct Solution {}

impl Solution {
    pub fn two_edit_words(queries: Vec<String>, dictionary: Vec<String>) -> Vec<String> {
        queries
            .into_iter()
            .filter(|q| {
                dictionary
                    .iter()
                    .find(|d| {
                        (0..q.len())
                            .filter(|i| d.as_bytes()[*i] != q.as_bytes()[*i])
                            .count()
                            < 3
                    })
                    .is_some()
            })
            .collect()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_2452() {
        assert_eq!(
            Solution::two_edit_words(
                vec![
                    "word".to_string(),
                    "note".to_string(),
                    "ants".to_string(),
                    "wood".to_string()
                ],
                vec!["wood".to_string(), "joke".to_string(), "moat".to_string()]
            ),
            vec!["word".to_string(), "note".to_string(), "wood".to_string()]
        );
        assert_eq!(
            Solution::two_edit_words(vec!["yes".to_string()], vec!["not".to_string()]),
            vec![] as Vec<String>
        );
    }
}
