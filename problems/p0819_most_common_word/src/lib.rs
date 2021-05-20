use std::collections::{BTreeMap, HashSet};

pub struct Solution {}

impl Solution {
    pub fn most_common_word(paragraph: String, banned: Vec<String>) -> String {
        let banned = banned.into_iter().collect::<HashSet<String>>();
        let mut count = paragraph
            .to_ascii_lowercase()
            .split(&['!', '?', '\'', ',', ';', '.', ' '][..])
            .fold(BTreeMap::new(), |mut map, w| {
                if !w.is_empty() && !banned.contains(w) {
                    *map.entry(w.to_string()).or_insert(0) += 1;
                }
                map
            })
            .into_iter()
            .collect::<Vec<(String, usize)>>();

        count.sort_unstable_by(|a, b| b.1.cmp(&a.1));
        count.remove(0).0
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_0819() {
        assert_eq!(
            Solution::most_common_word(
                "Bob hit a ball, the hit BALL flew far after it was hit.".to_string(),
                vec!["hit".to_string()]
            ),
            "ball".to_string()
        );

        assert_eq!(
            Solution::most_common_word("a, a, a, a, b,b,b,c, c".to_string(), vec!["a".to_string()]),
            "b".to_string()
        );
    }
}
