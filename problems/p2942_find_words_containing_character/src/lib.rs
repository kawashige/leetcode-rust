pub struct Solution {}

impl Solution {
    pub fn find_words_containing(words: Vec<String>, x: char) -> Vec<i32> {
        (0..words.len() as i32)
            .filter(|i| words[*i as usize].contains(x))
            .collect()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_2942() {
        assert_eq!(
            Solution::find_words_containing(vec!["leet".to_string(), "code".to_string()], 'e'),
            vec![0, 1]
        );
        assert_eq!(
            Solution::find_words_containing(
                vec![
                    "abc".to_string(),
                    "bcd".to_string(),
                    "aaaa".to_string(),
                    "cbc".to_string()
                ],
                'a'
            ),
            vec![0, 2]
        );
        assert_eq!(
            Solution::find_words_containing(
                vec![
                    "abc".to_string(),
                    "bcd".to_string(),
                    "aaaa".to_string(),
                    "cbc".to_string()
                ],
                'z'
            ),
            vec![]
        );
    }
}
