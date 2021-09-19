pub struct Solution {}

impl Solution {
    pub fn num_distinct(s: String, t: String) -> i32 {
        let s_bytes = s.as_bytes();
        let t_bytes = t.as_bytes();

        let mut dp = vec![vec![0; t.len()]; s.len()];

        for i in 0..s.len() {
            for j in 0..t.len() {
                if i > 0 && dp[i - 1][j] > 0 {
                    dp[i][j] += dp[i - 1][j];
                    if j + 1 < t.len() && s_bytes[i] == t_bytes[j + 1] {
                        dp[i][j + 1] += dp[i - 1][j];
                    }
                }
            }

            if s_bytes[i] == t_bytes[0] {
                dp[i][0] += 1;
            }
        }

        dp[s.len() - 1][t.len() - 1]
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_day20() {
        assert_eq!(
            Solution::num_distinct("rabbbit".to_string(), "rabbit".to_string()),
            3
        );
        assert_eq!(
            Solution::num_distinct("babgbag".to_string(), "bag".to_string()),
            5
        );
    }
}
