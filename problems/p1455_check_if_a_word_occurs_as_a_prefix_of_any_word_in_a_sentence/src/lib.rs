pub struct Solution {}

impl Solution {
    pub fn is_prefix_of_word(sentence: String, search_word: String) -> i32 {
        if let Some((_, i)) = sentence
            .split_ascii_whitespace()
            .zip(1..)
            .find(|(word, _)| word.starts_with(&search_word))
        {
            i
        } else {
            -1
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_1455() {
        assert_eq!(
            Solution::is_prefix_of_word("i love eating burger".to_string(), "burg".to_string()),
            4
        );
        assert_eq!(
            Solution::is_prefix_of_word(
                "this problem is an easy problem".to_string(),
                "pro".to_string()
            ),
            2
        );
        assert_eq!(
            Solution::is_prefix_of_word("i am tired".to_string(), "you".to_string()),
            -1
        );
    }
}
