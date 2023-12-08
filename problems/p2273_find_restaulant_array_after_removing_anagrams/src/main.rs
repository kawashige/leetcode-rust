pub struct Solution {}

impl Solution {
    pub fn sorted_char_string(s: &str) -> String {
        let mut chars = s.chars().collect::<Vec<_>>();
        chars.sort_unstable();
        chars.into_iter().collect()
    }
    pub fn remove_anagrams(words: Vec<String>) -> Vec<String> {
        let mut result = Vec::new();
        let mut sorted = Vec::new();

        sorted.push(Self::sorted_char_string(&words[0]));
        result.push(words[0].clone());

        for i in 1..words.len() {
            let sorted_string = Self::sorted_char_string(&words[i]);
            if sorted.last() != Some(&sorted_string) {
                result.push(words[i].clone());
                sorted.push(sorted_string);
            }
        }

        result
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_2273() {
        assert_eq!(
            Solution::remove_anagrams(vec![
                "abba".to_string(),
                "baba".to_string(),
                "bbaa".to_string(),
                "cd".to_string(),
                "cd".to_string()
            ]),
            vec!["abba".to_string(), "cd".to_string()]
        );
        assert_eq!(
            Solution::remove_anagrams(vec![
                "a".to_string(),
                "b".to_string(),
                "c".to_string(),
                "d".to_string(),
                "e".to_string()
            ]),
            vec![
                "a".to_string(),
                "b".to_string(),
                "c".to_string(),
                "d".to_string(),
                "e".to_string()
            ]
        );
    }
}
