pub struct Solution {}

impl Solution {
    pub fn get_longest_subsequence(words: Vec<String>, groups: Vec<i32>) -> Vec<String> {
        (0..words.len())
            .filter_map(|i| {
                if i == 0 || groups[i - 1] != groups[i] {
                    Some(words[i].clone())
                } else {
                    None
                }
            })
            .collect()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_2900() {
        assert_eq!(
            Solution::get_longest_subsequence(
                vec!["e".to_string(), "a".to_string(), "b".to_string()],
                vec![0, 0, 1]
            ),
            vec!["e".to_string(), "b".to_string()]
        );
        assert_eq!(
            Solution::get_longest_subsequence(
                vec![
                    "a".to_string(),
                    "b".to_string(),
                    "c".to_string(),
                    "d".to_string()
                ],
                vec![1, 0, 1, 1]
            ),
            vec!["a".to_string(), "b".to_string(), "c".to_string()]
        );
    }
}
