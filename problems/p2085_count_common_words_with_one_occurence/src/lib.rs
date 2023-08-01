use std::collections::HashMap;

pub struct Solution {}

impl Solution {
    pub fn count(words: Vec<String>) -> HashMap<String, usize> {
        words.into_iter().fold(HashMap::new(), |mut map, s| {
            *map.entry(s).or_insert(0) += 1;
            map
        })
    }

    pub fn count_words(words1: Vec<String>, words2: Vec<String>) -> i32 {
        let count1 = Self::count(words1);
        let count2 = Self::count(words2);

        count1
            .keys()
            .filter(|key| count1.get(*key).unwrap() == &1 && count2.get(*key).unwrap_or(&0) == &1)
            .count() as i32
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_2085() {
        assert_eq!(
            Solution::count_words(
                vec![
                    "leetcode".to_string(),
                    "is".to_string(),
                    "amazing".to_string(),
                    "as".to_string(),
                    "is".to_string()
                ],
                vec![
                    "amazing".to_string(),
                    "leetcode".to_string(),
                    "is".to_string()
                ]
            ),
            2
        );
        assert_eq!(
            Solution::count_words(
                vec!["b".to_string(), "bb".to_string(), "bbb".to_string()],
                vec!["a".to_string(), "aa".to_string(), "aaa".to_string()]
            ),
            0
        );
        assert_eq!(
            Solution::count_words(
                vec!["a".to_string(), "ab".to_string()],
                vec![
                    "a".to_string(),
                    "a".to_string(),
                    "a".to_string(),
                    "ab".to_string()
                ]
            ),
            1
        );
    }
}
