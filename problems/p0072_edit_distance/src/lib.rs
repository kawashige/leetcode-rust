pub struct Solution {}

impl Solution {
    pub fn min_distance(word1: String, word2: String) -> i32 {
        let mut dp = vec![vec![std::i32::MAX; word2.len() + 1]; word1.len() + 1];
        for i in 0..=word2.len() {
            dp[0][i] = i as i32;
        }
        for i in 0..=word1.len() {
            dp[i][0] = i as i32;
        }

        for i in 0..word1.len() {
            for j in 0..word2.len() {
                // do nothing or replace
                dp[i + 1][j + 1] = dp[i + 1][j + 1].min(
                    dp[i][j]
                        + if word1.as_bytes()[i] == word2.as_bytes()[j] {
                            0
                        } else {
                            1
                        },
                );
                // delete or insert
                dp[i + 1][j + 1] = dp[i + 1][j + 1].min(dp[i][j + 1] + 1);
                dp[i + 1][j + 1] = dp[i + 1][j + 1].min(dp[i + 1][j] + 1);
            }
        }

        dp[word1.len()][word2.len()]
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_0072() {
        assert_eq!(
            Solution::min_distance(
                "pneumonoultramicroscopicsilicovolcanoconiosis".to_string(),
                "ultramicroscopically".to_string()
            ),
            27
        );
        assert_eq!(
            Solution::min_distance("horse".to_string(), "ros".to_string()),
            3
        );
        assert_eq!(
            Solution::min_distance("intention".to_string(), "execution".to_string()),
            5
        );
    }
}
