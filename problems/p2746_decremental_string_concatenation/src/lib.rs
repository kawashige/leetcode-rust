pub struct Solution {}

impl Solution {
    pub fn minimize_concatenated_length(words: Vec<String>) -> i32 {
        let mut dp = vec![vec![vec![50_001; 26]; 26]; words.len()];
        dp[0][(words[0].as_bytes()[0] - b'a') as usize]
            [(words[0].as_bytes()[words[0].len() - 1] - b'a') as usize] = words[0].len() as i32;

        for i in 1..words.len() {
            let w_l = (words[i].as_bytes()[0] - b'a') as usize;
            let w_r = (words[i].as_bytes()[words[i].len() - 1] - b'a') as usize;

            for l in 0..26 {
                for r in 0..26 {
                    dp[i][w_l][r] = dp[i][w_l][r].min(
                        dp[i - 1][l][r] + words[i].len() as i32 - if w_r == l { 1 } else { 0 },
                    );
                    dp[i][l][w_r] = dp[i][l][w_r].min(
                        dp[i - 1][l][r] + words[i].len() as i32 - if w_l == r { 1 } else { 0 },
                    );
                }
            }
        }

        let mut result = std::i32::MAX;
        for l in 0..26 {
            for r in 0..26 {
                result = result.min(dp[dp.len() - 1][l][r]);
            }
        }
        result
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_2746() {
        assert_eq!(
            Solution::minimize_concatenated_length(vec![
                "aa".to_string(),
                "ab".to_string(),
                "bc".to_string()
            ]),
            4
        );
        assert_eq!(
            Solution::minimize_concatenated_length(vec!["ab".to_string(), "b".to_string()]),
            2
        );
        assert_eq!(
            Solution::minimize_concatenated_length(vec![
                "aaa".to_string(),
                "c".to_string(),
                "aba".to_string()
            ]),
            6
        );
    }
}
