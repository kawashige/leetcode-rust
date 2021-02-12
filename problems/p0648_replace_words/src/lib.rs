pub struct Solution {}

impl Solution {
    pub fn replace_words(mut dictionary: Vec<String>, sentence: String) -> String {
        dictionary.sort_unstable();
        let mut new_dictionary = Vec::new();
        for i in 0..dictionary.len() {
            if new_dictionary.is_empty()
                || !dictionary[i].starts_with(new_dictionary.last().unwrap())
            {
                new_dictionary.push(dictionary[i].clone());
            }
        }

        sentence
            .split_whitespace()
            .map(|w| {
                new_dictionary
                    .iter()
                    .find(|d| w.starts_with(*d))
                    .unwrap_or(&w.to_string())
                    .to_string()
            })
            .collect::<Vec<String>>()
            .join(" ")
    }
}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn test_0648() {
        assert_eq!(
            Solution::replace_words(
                vec!["cat".to_string(), "bat".to_string(), "rat".to_string()],
                "the cattle was rattled by the battery".to_string()
            ),
            "the cat was rat by the bat".to_string()
        );

        assert_eq!(
            Solution::replace_words(
                vec!["a".to_string(), "b".to_string(), "c".to_string()],
                "aadsfasf absbs bbab cadsfafs".to_string()
            ),
            "a a b c".to_string()
        );

        assert_eq!(
            Solution::replace_words(
                vec![
                    "a".to_string(),
                    "aa".to_string(),
                    "aaa".to_string(),
                    "aaaa".to_string()
                ],
                "a aa a aaaa aaa aaa aaa aaaaaa bbb baba ababa".to_string()
            ),
            "a a a a a a a a bbb baba a".to_string()
        );

        assert_eq!(
            Solution::replace_words(
                vec![
                    "catt".to_string(),
                    "cat".to_string(),
                    "bat".to_string(),
                    "rat".to_string()
                ],
                "the cattle was rattled by the battery".to_string()
            ),
            "the cat was rat by the bat".to_string()
        );

        assert_eq!(
            Solution::replace_words(
                vec!["ac".to_string(), "ab".to_string()],
                "it is abnormal that this solution is accepted".to_string()
            ),
            "it is ab that this solution is ac".to_string()
        );
    }
}
