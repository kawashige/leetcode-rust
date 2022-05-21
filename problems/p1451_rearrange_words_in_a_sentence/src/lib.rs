pub struct Solution {}

impl Solution {
    pub fn arrange_words(text: String) -> String {
        let mut words = text
            .split_ascii_whitespace()
            .enumerate()
            .map(|(i, word)| (word.len(), i, word))
            .collect::<Vec<_>>();
        words.sort_unstable();
        let words = words
            .into_iter()
            .enumerate()
            .map(|(i, (_, _, word))| {
                if i == 0 {
                    word.chars()
                        .enumerate()
                        .map(|(j, c)| if j == 0 { c.to_ascii_uppercase() } else { c })
                        .collect()
                } else {
                    word.to_ascii_lowercase()
                }
            })
            .collect::<Vec<_>>();
        words.join(" ")
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_1451() {
        assert_eq!(
            Solution::arrange_words("Leetcode is cool".to_string()),
            "Is cool leetcode".to_string()
        );
        assert_eq!(
            Solution::arrange_words("Keep calm and code on".to_string()),
            "On and keep calm code".to_string()
        );
        assert_eq!(
            Solution::arrange_words("To be or not to be".to_string()),
            "To be or to be not".to_string()
        );
    }
}
