pub struct Solution {}

impl Solution {
    pub fn min_deletion_size(strs: Vec<String>) -> i32 {
        let mut dp = vec![vec![101; strs[0].len() + 1]; strs[0].len() + 1];
        dp[0][0] = 0;

        for i in 0..strs[0].len() {
            dp[i + 1][i + 1] = dp[i + 1][i + 1].min(i);
            for j in 0..dp[i + 1].len() {
                dp[i + 1][j] = dp[i + 1][j].min(dp[i][j]);
                if j < i && (0..strs.len()).all(|k| strs[k].as_bytes()[j] <= strs[k].as_bytes()[i])
                {
                    dp[i + 1][i + 1] = dp[i + 1][i + 1].min(dp[i][j + 1] + i - j - 1);
                }
            }
        }

        let mut min_deletion = strs[0].len();
        for i in 0..strs[0].len() {
            min_deletion = min_deletion.min(dp[dp.len() - 1][i + 1] + strs[0].len() - (i + 1));
        }

        min_deletion as i32
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_0960() {
        assert_eq!(
            Solution::min_deletion_size(vec!["babca".to_string(), "bbazb".to_string()]),
            3
        );
        assert_eq!(Solution::min_deletion_size(vec!["edcba".to_string()]), 4);
        assert_eq!(
            Solution::min_deletion_size(vec![
                "ghi".to_string(),
                "def".to_string(),
                "abc".to_string()
            ]),
            0
        );
    }
}
