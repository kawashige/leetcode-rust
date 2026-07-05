pub struct Solution {}

impl Solution {
    pub fn paths_with_max_score(board: Vec<String>) -> Vec<i32> {
        const M: i64 = 1_000_000_007;
        let mut dp = vec![vec![vec![-1, 0]; board[0].len()]; board.len()];
        dp[0][0] = vec![0, 0];
        dp[board.len() - 1][board[0].len() - 1] = vec![0, 1];

        for i in (0..board.len()).rev() {
            for j in (0..board[0].len()).rev() {
                if board[i].as_bytes()[j] == b'X' || dp[i][j][0] == -1 {
                    continue;
                }
                let val = if board[i].as_bytes()[j] != b'S' {
                    (board[i].as_bytes()[j] - b'0') as i64
                } else {
                    0
                };
                if 0 < i && board[i - 1].as_bytes()[j] != b'X' {
                    if dp[i - 1][j][0] < dp[i][j][0] + val {
                        dp[i - 1][j] = vec![dp[i][j][0] + val, dp[i][j][1]];
                    } else if dp[i - 1][j][0] == dp[i][j][0] + val {
                        dp[i - 1][j][1] += dp[i][j][1];
                        dp[i - 1][j][1] %= M;
                    }
                }
                if 0 < j && board[i].as_bytes()[j - 1] != b'X' {
                    if dp[i][j - 1][0] < dp[i][j][0] + val {
                        dp[i][j - 1] = vec![dp[i][j][0] + val, dp[i][j][1]];
                    } else if dp[i][j - 1][0] == dp[i][j][0] + val {
                        dp[i][j - 1][1] += dp[i][j][1];
                        dp[i][j - 1][1] %= M;
                    }
                }
                if 0 < i && 0 < j && board[i - 1].as_bytes()[j - 1] != b'X' {
                    if dp[i - 1][j - 1][0] < dp[i][j][0] + val {
                        dp[i - 1][j - 1] = vec![dp[i][j][0] + val, dp[i][j][1]];
                    } else if dp[i - 1][j - 1][0] == dp[i][j][0] + val {
                        dp[i - 1][j - 1][1] += dp[i][j][1];
                        dp[i - 1][j - 1][1] %= M;
                    }
                }
            }
        }

        vec![dp[0][0][0] as i32, dp[0][0][1] as i32]
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_1301() {
        assert_eq!(
            Solution::paths_with_max_score(vec![
                "E23".to_string(),
                "2X2".to_string(),
                "12S".to_string()
            ]),
            vec![7, 1]
        );
        assert_eq!(
            Solution::paths_with_max_score(vec![
                "E12".to_string(),
                "1X1".to_string(),
                "21S".to_string()
            ]),
            vec![4, 2]
        );
        assert_eq!(
            Solution::paths_with_max_score(vec![
                "E11".to_string(),
                "XXX".to_string(),
                "11S".to_string()
            ]),
            vec![0, 0]
        );
    }
}

fn main() {}
