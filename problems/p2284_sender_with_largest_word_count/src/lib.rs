use std::collections::HashMap;

pub struct Solution {}

impl Solution {
    pub fn largest_word_count(messages: Vec<String>, senders: Vec<String>) -> String {
        let mut count = (0..messages.len())
            .fold(HashMap::new(), |mut count, i| {
                *count.entry(senders[i].clone()).or_insert(0) +=
                    messages[i].split_ascii_whitespace().count();
                count
            })
            .into_iter()
            .map(|(key, val)| (val, key))
            .collect::<Vec<_>>();
        count.sort_unstable();
        count.last().unwrap().1.to_string()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_2284() {
        assert_eq!(
            Solution::largest_word_count(
                vec![
                    "Hello userTwooo".to_string(),
                    "Hi userThree".to_string(),
                    "Wonderful day Alice".to_string(),
                    "Nice day userThree".to_string()
                ],
                vec![
                    "Alice".to_string(),
                    "userTwo".to_string(),
                    "userThree".to_string(),
                    "Alice".to_string()
                ]
            ),
            "Alice".to_string()
        );
        assert_eq!(
            Solution::largest_word_count(
                vec![
                    "How is leetcode for everyone".to_string(),
                    "Leetcode is useful for practice".to_string()
                ],
                vec!["Bob".to_string(), "Charlie".to_string()]
            ),
            "Charlie".to_string()
        );
    }
}
