pub struct Solution {}

impl Solution {
    pub fn num_ways(words: Vec<String>, target: String) -> i32 {
        const M: usize = 1_000_000_007;

        let mut chars = vec![[0; 26]; words[0].len()];
        for i in 0..words.len() {
            for j in 0..words[i].len() {
                chars[j][(words[i].as_bytes()[j] - b'a') as usize] += 1;
            }
        }

        let mut dp = vec![vec![0; target.len() + 1]; words[0].len() + 1];
        dp[0][0] = 1;

        for i in 0..words[0].len() {
            dp[i + 1][0] = 1;
            for j in 1..dp[i].len() {
                dp[i + 1][j] = dp[i][j];
                if 0 < dp[i][j - 1] {
                    dp[i + 1][j] +=
                        dp[i][j - 1] * chars[i][(target.as_bytes()[j - 1] - b'a') as usize] % M;
                }
                dp[i + 1][j] %= M;
            }
        }

        *dp.last().unwrap().last().unwrap() as i32
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_1639() {
        assert_eq!(
            Solution::num_ways(
                vec!["acca".to_string(), "bbbb".to_string(), "caca".to_string()],
                "aba".to_string()
            ),
            6
        );
        assert_eq!(
            Solution::num_ways(
                vec!["abba".to_string(), "baab".to_string()],
                "bab".to_string()
            ),
            4
        );
    }
}
