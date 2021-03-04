pub struct Solution {}

impl Solution {
    pub fn minimum_delete_sum(s1: String, s2: String) -> i32 {
        let bytes1 = s1.as_bytes();
        let bytes2 = s2.as_bytes();

        let mut dp = vec![vec![0; s2.len() + 1]; s1.len() + 1];
        for i in 0..bytes1.len() {
            dp[i + 1][0] = dp[i][0] + bytes1[i] as i32;
        }
        for i in 0..bytes2.len() {
            dp[0][i + 1] = dp[0][i] + bytes2[i] as i32;
        }

        for i in 0..bytes1.len() {
            for j in 0..bytes2.len() {
                dp[i + 1][j + 1] = if bytes1[i] == bytes2[j] {
                    dp[i][j]
                } else {
                    std::cmp::min(
                        dp[i][j + 1] + bytes1[i] as i32,
                        dp[i + 1][j] + bytes2[j] as i32,
                    )
                }
            }
        }

        dp[bytes1.len()][bytes2.len()]
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_0712() {
        assert_eq!(
            Solution::minimum_delete_sum("ccaccjp".to_string(), "fwosarcwge".to_string()),
            1399
        );
        assert_eq!(
            Solution::minimum_delete_sum("sea".to_string(), "eat".to_string()),
            231
        );
        assert_eq!(
            Solution::minimum_delete_sum("delete".to_string(), "leet".to_string()),
            403
        );
    }
}
