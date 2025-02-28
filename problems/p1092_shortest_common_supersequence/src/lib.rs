pub struct Solution {}

impl Solution {
    pub fn shortest_common_supersequence(str1: String, str2: String) -> String {
        let mut dp = vec![vec![0; str2.len() + 1]; str1.len() + 1];
        for i in (0..str1.len()).rev() {
            for j in (0..str2.len()).rev() {
                if str1.as_bytes()[i] == str2.as_bytes()[j] {
                    dp[i][j] = dp[i + 1][j + 1] + 1;
                } else {
                    dp[i][j] = dp[i][j + 1].max(dp[i + 1][j]);
                }
            }
        }
        let mut result = String::new();
        let mut i1 = 0;
        let mut i2 = 0;
        while i1 < str1.len() && i2 < str2.len() {
            if str1.as_bytes()[i1] == str2.as_bytes()[i2] {
                result.push(str1.as_bytes()[i1] as char);
                i1 += 1;
                i2 += 1;
            } else if dp[i1 + 1][i2] > dp[i1][i2 + 1] {
                result.push(str1.as_bytes()[i1] as char);
                i1 += 1;
            } else {
                result.push(str2.as_bytes()[i2] as char);
                i2 += 1;
            }
        }
        while i1 < str1.len() {
            result.push(str1.as_bytes()[i1] as char);
            i1 += 1;
        }
        while i2 < str2.len() {
            result.push(str2.as_bytes()[i2] as char);
            i2 += 1;
        }
        result
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_1092() {
        assert_eq!(
            Solution::shortest_common_supersequence("abac".to_string(), "cab".to_string()),
            "cabac".to_string()
        );
        assert_eq!(
            Solution::shortest_common_supersequence("aaaaaaaa".to_string(), "aaaaaaaa".to_string()),
            "aaaaaaaa".to_string()
        );
    }
}
