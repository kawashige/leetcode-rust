use std::collections::HashSet;
pub struct Solution {}

impl Solution {
    pub fn longest_word(mut words: Vec<String>) -> String {
        words.sort_unstable_by(|a, b| b.len().cmp(&a.len()).then(a.cmp(&b)));
        let set = words.iter().map(|s| s.as_str()).collect::<HashSet<&str>>();
        for w in &words {
            if (1..w.len()).all(|i| set.contains(&w[..i])) {
                return w.to_string();
            }
        }
        "".to_string()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_0720() {
        assert_eq!(
            Solution::longest_word(vec!["".to_string(),]),
            "".to_string()
        );
        assert_eq!(
            Solution::longest_word(vec![
                "k".to_string(),
                "lg".to_string(),
                "it".to_string(),
                "oidd".to_string(),
                "oid".to_string(),
                "oiddm".to_string(),
                "kfk".to_string(),
                "y".to_string(),
                "mw".to_string(),
                "kf".to_string(),
                "l".to_string(),
                "o".to_string(),
                "mwaqz".to_string(),
                "oi".to_string(),
                "ych".to_string(),
                "m".to_string(),
                "mwa".to_string()
            ]),
            "oiddm".to_string()
        );
        assert_eq!(
            Solution::longest_word(vec![
                "w".to_string(),
                "wo".to_string(),
                "wor".to_string(),
                "worl".to_string(),
                "world".to_string()
            ]),
            "world".to_string()
        );
        assert_eq!(
            Solution::longest_word(vec![
                "a".to_string(),
                "banana".to_string(),
                "app".to_string(),
                "appl".to_string(),
                "ap".to_string(),
                "apply".to_string(),
                "apple".to_string()
            ]),
            "apple".to_string()
        );
    }
}
