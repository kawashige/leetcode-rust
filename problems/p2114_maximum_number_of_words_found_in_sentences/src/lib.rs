pub struct Solution {}

impl Solution {
    pub fn most_words_found(sentences: Vec<String>) -> i32 {
        sentences
            .into_iter()
            .map(|s| s.split_whitespace().count() as i32)
            .max()
            .unwrap()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_2114() {
        assert_eq!(
            Solution::most_words_found(vec![
                "alice and bob love leetcode".to_string(),
                "i think so too".to_string(),
                "this is great thanks very much".to_string()
            ]),
            6
        );
        assert_eq!(
            Solution::most_words_found(vec![
                "please wait".to_string(),
                "continue to fight".to_string(),
                "continue to win".to_string()
            ]),
            3
        );
    }
}
