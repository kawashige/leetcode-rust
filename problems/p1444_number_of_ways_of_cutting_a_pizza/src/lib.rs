pub struct Solution {}

impl Solution {
    pub fn ways(pizza: Vec<String>, k: i32) -> i32 {
        const M: usize = 1_000_000_007;

        let mut count = vec![vec![0; pizza[0].len()]; pizza.len()];
        for i in (0..pizza.len()).rev() {
            for j in (0..pizza[0].len()).rev() {
                count[i][j] = if pizza[i].as_bytes()[j] == b'A' { 1 } else { 0 };
                if i < pizza.len() - 1 {
                    count[i][j] += count[i + 1][j];
                }
                if j < pizza[0].len() - 1 {
                    count[i][j] += count[i][j + 1];
                }
                if i < pizza.len() - 1 && j < pizza[0].len() - 1 {
                    count[i][j] -= count[i + 1][j + 1];
                }
            }
        }

        let mut dp = vec![vec![vec![0; k as usize]; pizza[0].len()]; pizza.len()];
        dp[0][0][0] = 1;
        let mut result = 0;

        for i in 0..pizza.len() {
            for j in 0..pizza[0].len() {
                if count[i][j] == 0 {
                    continue;
                }
                for v in 0..i {
                    if 0 < count[v][j] - count[i][j] {
                        for l in 1..k as usize {
                            dp[i][j][l] += dp[v][j][l - 1];
                            dp[i][j][l] %= M;
                        }
                    }
                }
                for c in 0..j {
                    if 0 < count[i][c] - count[i][j] {
                        for l in 1..k as usize {
                            dp[i][j][l] += dp[i][c][l - 1];
                            dp[i][j][l] %= M;
                        }
                    }
                }
                result += dp[i][j][k as usize - 1];
                result %= M;
            }
        }

        result as i32
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_1444() {
        assert_eq!(
            Solution::ways(
                vec![".A".to_string(), "AA".to_string(), "A.".to_string()],
                3
            ),
            3
        );
        assert_eq!(
            Solution::ways(
                vec!["A..".to_string(), "AAA".to_string(), "...".to_string()],
                3
            ),
            3
        );
        assert_eq!(
            Solution::ways(
                vec!["A..".to_string(), "AA.".to_string(), "...".to_string()],
                3
            ),
            1
        );
        assert_eq!(
            Solution::ways(
                vec!["A..".to_string(), "A..".to_string(), "...".to_string()],
                1
            ),
            1
        );
    }
}
