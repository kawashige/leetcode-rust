pub struct Solution {}

impl Solution {
    pub fn min_distance(word1: String, word2: String) -> i32 {
        let bytes1 = word1.as_bytes();
        let bytes2 = word2.as_bytes();

        let mut dp = vec![vec![0; bytes2.len() + 1]; bytes1.len() + 1];

        for i in 0..bytes1.len() {
            for j in 0..bytes2.len() {
                if bytes1[i] == bytes2[j] {
                    dp[i + 1][j + 1] = dp[i][j] + 1;
                } else {
                    dp[i + 1][j + 1] = std::cmp::max(dp[i][j + 1], dp[i + 1][j]);
                }
            }
        }

        bytes1.len() as i32 + bytes2.len() as i32 - dp[bytes1.len()][bytes2.len()] * 2
    }
}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn test_0583() {
        assert_eq!(
            Solution::min_distance("plasma".to_string(), "altruism".to_string()),
            8
        );
        assert_eq!(
            Solution::min_distance("xaxbxcxdxexf".to_string(), "yaybycydyeyf".to_string()),
            12
        );
        assert_eq!(
            Solution::min_distance("sea".to_string(), "ate".to_string()),
            4
        );
        assert_eq!(
            Solution::min_distance("sea".to_string(), "eat".to_string()),
            2
        );
        assert_eq!(
            Solution::min_distance("sea".to_string(), "sea".to_string()),
            0
        );
    }
}
