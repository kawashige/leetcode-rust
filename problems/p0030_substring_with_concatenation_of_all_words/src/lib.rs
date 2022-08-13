use std::collections::HashMap;

pub struct Solution {}

impl Solution {
    pub fn find_substring(s: String, words: Vec<String>) -> Vec<i32> {
        let words_len = words.len();
        let word_len = words[0].len();

        if s.len() < word_len * words_len {
            return Default::default();
        }

        let word_map = words.into_iter().fold(HashMap::new(), |mut map, word| {
            *map.entry(word).or_insert(0) += 1;
            map
        });
        let mut word_count = vec![0; word_map.len()];
        let mut word_indices = HashMap::new();
        for (i, (word, count)) in word_map.into_iter().enumerate() {
            word_count[i] = count;
            word_indices.insert(word, i);
        }

        let mut result = Vec::new();
        for i in 0..=s.len() - word_len * words_len {
            let mut count = vec![0; word_count.len()];
            let mut is_ok = true;
            for j in 0..words_len {
                if let Some(k) =
                    word_indices.get(&s[(i + j * word_len)..(i + j * word_len + word_len)])
                {
                    count[*k] += 1;
                    if word_count[*k] < count[*k] {
                        is_ok = false;
                        break;
                    }
                } else {
                    is_ok = false;
                    break;
                }
            }

            if is_ok {
                result.push(i as i32);
            }
        }

        result
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_0030() {
        assert_eq!(
            Solution::find_substring("a".to_string(), vec!["a".to_string(), "a".to_string()]),
            vec![]
        );
        assert_eq!(
            Solution::find_substring(
                "wordgoodgoodgoodbestword".to_string(),
                vec![
                    "word".to_string(),
                    "good".to_string(),
                    "best".to_string(),
                    "good".to_string()
                ]
            ),
            vec![8]
        );
        assert_eq!(
            Solution::find_substring(
                "barfoothefoobarman".to_string(),
                vec!["foo".to_string(), "bar".to_string()]
            ),
            vec![0, 9]
        );
        assert_eq!(
            Solution::find_substring(
                "wordgoodgoodgoodbestword".to_string(),
                vec![
                    "word".to_string(),
                    "good".to_string(),
                    "best".to_string(),
                    "word".to_string()
                ]
            ),
            vec![]
        );
        assert_eq!(
            Solution::find_substring(
                "barfoofoobarthefoobarman".to_string(),
                vec!["bar".to_string(), "foo".to_string(), "the".to_string()]
            ),
            vec![6, 9, 12]
        );
    }
}
