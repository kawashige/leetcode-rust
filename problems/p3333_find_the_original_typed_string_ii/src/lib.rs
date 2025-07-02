pub struct Solution {}

impl Solution {
    pub fn possible_string_count(word: String, k: i32) -> i32 {
        const M: i64 = 1_000_000_007;

        let mut counts = Vec::new();
        let mut count = 1;
        for i in 1..word.len() {
            if word.as_bytes()[i - 1] != word.as_bytes()[i] {
                counts.push(count);
                count = 1;
            } else {
                count += 1;
            }
        }
        counts.push(count);

        let mut minus = 0;
        if counts.len() < k as usize {
            let mut dp = vec![vec![0; k as usize]; counts.len() + 1];
            dp[0][0] = 1;

            for i in 0..counts.len() {
                for j in 0..dp[i].len() - 1 {
                    if 0 < dp[i][j] {
                        dp[i + 1][j + 1] += dp[i][j];
                        dp[i + 1][j + 1] %= M;
                        if j + counts[i] + 1 < dp[i].len() {
                            dp[i + 1][j + counts[i] + 1] -= dp[i][j];
                        }
                    }
                }
                for j in 1..dp[i].len() {
                    dp[i + 1][j] += dp[i + 1][j - 1];
                    dp[i + 1][j] %= M;
                    if dp[i + 1][j] < 0 {
                        dp[i + 1][j] += M;
                    }
                }
            }

            minus = dp
                .last()
                .unwrap()
                .into_iter()
                .fold(0, |acc, x| (acc + x) % M);
        }

        let mut result = counts.iter().fold(1, |acc, x| (acc * *x as i64) % M);
        result -= minus;
        while result < 0 {
            result += M
        }
        result as i32
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_3333() {
        assert_eq!(
            Solution::possible_string_count("aabbccdd".to_string(), 7),
            5
        );
        assert_eq!(
            Solution::possible_string_count("aabbccdd".to_string(), 8),
            1
        );
        assert_eq!(Solution::possible_string_count("aaabbb".to_string(), 3), 8);
    }
}
