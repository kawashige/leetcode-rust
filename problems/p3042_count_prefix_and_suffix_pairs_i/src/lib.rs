pub struct Solution {}

impl Solution {
    pub fn count_prefix_suffix_pairs(words: Vec<String>) -> i32 {
        let mut count = 0;

        for i in 0..words.len() {
            for j in 0..i {
                if words[i].starts_with(&words[j]) && words[i].ends_with(&words[j]) {
                    count += 1;
                }
            }
        }

        count
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_3042() {
        assert_eq!(
            Solution::count_prefix_suffix_pairs(vec![
                "a".to_string(),
                "aba".to_string(),
                "ababa".to_string(),
                "aa".to_string()
            ]),
            4
        );
        assert_eq!(
            Solution::count_prefix_suffix_pairs(vec![
                "pa".to_string(),
                "papa".to_string(),
                "ma".to_string(),
                "mama".to_string()
            ]),
            2
        );
        assert_eq!(
            Solution::count_prefix_suffix_pairs(vec!["abab".to_string(), "ab".to_string()]),
            0
        );
    }
}
