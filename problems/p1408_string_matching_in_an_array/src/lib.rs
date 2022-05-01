pub struct Solution {}

impl Solution {
    pub fn string_matching(mut words: Vec<String>) -> Vec<String> {
        words.sort_unstable_by_key(|s| s.len());

        let mut result = Vec::new();
        for i in 0..words.len() {
            for j in (i + 1)..words.len() {
                if words[j].contains(&words[i]) {
                    result.push(words[i].clone());
                    break;
                }
            }
        }

        result
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_1408() {
        assert_eq!(
            Solution::string_matching(vec![
                "mass".to_string(),
                "as".to_string(),
                "hero".to_string(),
                "superhero".to_string()
            ]),
            vec!["as".to_string(), "hero".to_string()]
        );
        assert_eq!(
            Solution::string_matching(vec![
                "leetcode".to_string(),
                "et".to_string(),
                "code".to_string()
            ]),
            vec!["et".to_string(), "code".to_string()]
        );
        assert_eq!(
            Solution::string_matching(vec![
                "blue".to_string(),
                "green".to_string(),
                "bu".to_string()
            ]),
            vec![] as Vec<String>
        );
    }
}
