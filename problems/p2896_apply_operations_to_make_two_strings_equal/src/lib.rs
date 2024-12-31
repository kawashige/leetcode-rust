pub struct Solution {}

impl Solution {
    pub fn min_operations(s1: String, s2: String, x: i32) -> i32 {
        let indices = (0..s1.len())
            .filter(|i| s1.as_bytes()[*i] != s2.as_bytes()[*i])
            .collect::<Vec<_>>();
        if indices.len() % 2 == 1 {
            return -1;
        }
        if indices.is_empty() {
            return 0;
        }

        let mut dp = vec![vec![std::i32::MAX; indices.len()]; indices.len()];
        for i in 0..indices.len() {
            dp[i][i] = 0;
        }
        for l in 1..indices.len() {
            for i in 0..indices.len() - l {
                dp[i][i + l] = dp[i][i + l].min(
                    ((indices[i + 1] - indices[i]) as i32).min(x)
                        + if 1 < l { dp[i + 1 + 1][i + l] } else { 0 },
                );
                dp[i][i + l] = dp[i][i + l].min(
                    ((indices[i + l] - indices[i + l - 1]) as i32).min(x)
                        + if 1 < l { dp[i][i + l - 1 - 1] } else { 0 },
                );
                dp[i][i + l] = dp[i][i + l].min(
                    ((indices[i + l] - indices[i]) as i32).min(x)
                        + if 2 < l { dp[i + 1][i + l - 1] } else { 0 },
                );
            }
        }

        dp[0][indices.len() - 1]
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_2896() {
        assert_eq!(
            Solution::min_operations("101101".to_string(), "000000".to_string(), 6),
            4
        );
        assert_eq!(
            Solution::min_operations("1100011000".to_string(), "0101001010".to_string(), 2),
            4
        );
        assert_eq!(
            Solution::min_operations("10110".to_string(), "00011".to_string(), 4),
            -1
        );
    }
}
