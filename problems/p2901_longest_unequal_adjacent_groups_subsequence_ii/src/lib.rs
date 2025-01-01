pub struct Solution {}

impl Solution {
    pub fn get_words_in_longest_subsequence(words: Vec<String>, groups: Vec<i32>) -> Vec<String> {
        let mut dp = vec![(1, words.len()); words.len()];
        let mut max = 0;
        let mut max_i = 0;
        for i in 0..words.len() {
            for j in 0..i {
                let dist = if words[i].len() != words[j].len() {
                    0
                } else {
                    (0..words[i].len())
                        .filter(|k| words[i].as_bytes()[*k] != words[j].as_bytes()[*k])
                        .count()
                };
                if dist != 1 || groups[i] == groups[j] {
                    continue;
                }
                if dp[i].0 < dp[j].0 + 1 {
                    dp[i] = (dp[j].0 + 1, j);
                    if max < dp[i].0 {
                        max = dp[i].0;
                        max_i = i;
                    }
                }
            }
        }

        let mut result = Vec::new();
        while max_i != words.len() {
            result.push(max_i);
            max_i = dp[max_i].1;
        }
        result.into_iter().rev().map(|i| words[i].clone()).collect()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_2901() {
        assert_eq!(
            Solution::get_words_in_longest_subsequence(
                vec!["bab".to_string(), "dab".to_string(), "cab".to_string()],
                vec![1, 2, 2]
            ),
            vec!["bab".to_string(), "dab".to_string()]
        );
        assert_eq!(
            Solution::get_words_in_longest_subsequence(
                vec![
                    "a".to_string(),
                    "b".to_string(),
                    "c".to_string(),
                    "d".to_string()
                ],
                vec![1, 2, 3, 4]
            ),
            vec![
                "a".to_string(),
                "b".to_string(),
                "c".to_string(),
                "d".to_string()
            ]
        );
    }
}
