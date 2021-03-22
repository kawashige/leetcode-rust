use std::collections::HashMap;

pub struct Solution {}

impl Solution {
    pub fn spellchecker(wordlist: Vec<String>, queries: Vec<String>) -> Vec<String> {
        let mut original = HashMap::new();
        let mut case_insensitive = HashMap::new();
        let mut error = HashMap::new();

        let is_vowel = |c: char| ['a', 'e', 'i', 'o', 'u'].contains(&c);

        for w in &wordlist {
            let case_insensitive_key = w.to_ascii_lowercase();
            let error_key = case_insensitive_key.replace(is_vowel, "*");

            original.entry(w.to_string()).or_insert(w);
            case_insensitive.entry(case_insensitive_key).or_insert(w);
            error.entry(error_key).or_insert(w);
        }

        queries
            .into_iter()
            .map(|q| {
                let case_insensitive_key = q.to_ascii_lowercase();
                let error_key = case_insensitive_key.replace(is_vowel, "*");

                original
                    .get(&q)
                    .unwrap_or(
                        case_insensitive
                            .get(&case_insensitive_key)
                            .unwrap_or(error.get(&error_key).unwrap_or(&&"".to_string())),
                    )
                    .to_string()
            })
            .collect()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_day22() {
        assert_eq!(
            Solution::spellchecker(
                vec![
                    "KiTe".to_string(),
                    "kite".to_string(),
                    "hare".to_string(),
                    "Hare".to_string()
                ],
                vec![
                    "kite".to_string(),
                    "Kite".to_string(),
                    "KiTe".to_string(),
                    "Hare".to_string(),
                    "HARE".to_string(),
                    "Hear".to_string(),
                    "hear".to_string(),
                    "keti".to_string(),
                    "keet".to_string(),
                    "keto".to_string()
                ]
            ),
            vec![
                "kite".to_string(),
                "KiTe".to_string(),
                "KiTe".to_string(),
                "Hare".to_string(),
                "hare".to_string(),
                "".to_string(),
                "".to_string(),
                "KiTe".to_string(),
                "".to_string(),
                "KiTe".to_string()
            ]
        );
    }
}
