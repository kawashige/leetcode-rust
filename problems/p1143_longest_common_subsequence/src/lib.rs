pub struct Solution {}

impl Solution {
    pub fn longest_common_subsequence(text1: String, text2: String) -> i32 {
        let bytes1 = text1.as_bytes();
        let bytes2 = text2.as_bytes();

        let mut dp = vec![vec![0; text2.len() + 1]; text1.len() + 1];

        for i in 0..bytes1.len() {
            for j in 0..bytes2.len() {
                dp[i + 1][j + 1] = if bytes1[i] == bytes2[j] {
                    dp[i][j] + 1
                } else {
                    dp[i][j + 1].max(dp[i + 1][j])
                };
            }
        }

        dp[bytes1.len()][bytes2.len()]
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_1143() {
        assert_eq!(
            Solution::longest_common_subsequence("abcde".to_string(), "ace".to_string()),
            3
        );
        assert_eq!(
            Solution::longest_common_subsequence("abc".to_string(), "abc".to_string()),
            3
        );
        assert_eq!(
            Solution::longest_common_subsequence("abc".to_string(), "def".to_string()),
            0
        );
    }
}
