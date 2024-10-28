pub struct Solution {}

impl Solution {
    pub fn split_words_by_separator(words: Vec<String>, separator: char) -> Vec<String> {
        words
            .into_iter()
            .map(|w| {
                w.split(separator)
                    .filter(|w| !w.is_empty())
                    .map(|w| w.to_string())
                    .collect::<Vec<_>>()
            })
            .flatten()
            .collect()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_2788() {
        assert_eq!(
            Solution::split_words_by_separator(
                vec![
                    "one.two.three".to_string(),
                    "four.five".to_string(),
                    "six".to_string()
                ],
                '.'
            ),
            vec![
                "one".to_string(),
                "two".to_string(),
                "three".to_string(),
                "four".to_string(),
                "five".to_string(),
                "six".to_string()
            ]
        );
        assert_eq!(
            Solution::split_words_by_separator(
                vec!["$easy$".to_string(), "$problem$".to_string()],
                '$'
            ),
            vec!["easy".to_string(), "problem".to_string()]
        );
        assert_eq!(
            Solution::split_words_by_separator(vec!["|||".to_string()], '|'),
            vec![] as Vec<String>
        );
    }
}
