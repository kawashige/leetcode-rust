pub struct Solution {}

impl Solution {
    pub fn min_insertions(s: String) -> i32 {
        let mut dp = vec![vec![0; s.len()]; s.len()];
        for i in 0..s.len() {
            dp[i][i] = 1;
        }

        for l in 1..s.len() {
            for i in 0..s.len() - l {
                if s.as_bytes()[i] == s.as_bytes()[i + l] {
                    dp[i][i + l] = if l == 1 { 0 } else { dp[i + 1][i + l - 1] } + 2;
                } else {
                    dp[i][i + l] = dp[i + 1][i + l].max(dp[i][i + l - 1]);
                }
            }
        }

        (s.len() - dp[0][s.len() - 1]) as i32
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_1312() {
        assert_eq!(Solution::min_insertions("zzazz".to_string()), 0);
        assert_eq!(Solution::min_insertions("mbadm".to_string()), 2);
        assert_eq!(Solution::min_insertions("leetcode".to_string()), 5);
    }
}
