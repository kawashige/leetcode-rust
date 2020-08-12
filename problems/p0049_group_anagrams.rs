pub struct Solution {}

impl Solution {
    pub fn group_anagrams(strs: Vec<String>) -> Vec<Vec<String>> {
        use std::collections::HashMap;
        let mut results = HashMap::new();
        for s in strs {
            let mut key = s.chars().collect::<Vec<char>>();
            key.sort();
            results
                .entry(key.iter().collect::<String>())
                .or_insert(Vec::new())
                .push(s);
        }
        results.values().cloned().collect::<Vec<Vec<String>>>()
    }
}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn test_0049() {
        let mut expected = vec![
            vec!["bat".to_string()],
            vec!["eat".to_string(), "tea".to_string(), "ate".to_string()],
            vec!["tan".to_string(), "nat".to_string()],
        ];
        expected.sort();
        let mut result = Solution::group_anagrams(vec![
            "eat".to_string(),
            "tea".to_string(),
            "tan".to_string(),
            "ate".to_string(),
            "nat".to_string(),
            "bat".to_string(),
        ]);
        result.sort();
        assert_eq!(expected, result);
    }
}
