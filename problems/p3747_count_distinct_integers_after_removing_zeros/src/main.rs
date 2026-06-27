pub struct Solution {}

impl Solution {
    pub fn count_distinct(n: i64) -> i64 {
        let digits = n
            .to_string()
            .as_bytes()
            .iter()
            .map(|b| (b - b'0') as usize)
            .collect::<Vec<_>>();
        let mut dp = vec![vec![0; 2]; digits.len() + 1];
        dp[0][1] = 1;

        for i in 0..digits.len() {
            for j in 0..2 {
                for k in 1..=if j == 0 { 9 } else { digits[i] } {
                    dp[i + 1][if j == 0 || k < digits[i] { 0 } else { 1 }] += dp[i][j];
                }
            }
        }

        let mut result = 0;
        let mut x = 9;
        for _ in 1..digits.len() {
            result += x;
            x *= 9;
        }

        result + dp.last().unwrap().into_iter().sum::<i64>()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_3747() {
        assert_eq!(Solution::count_distinct(10), 9);
        assert_eq!(Solution::count_distinct(3), 3);
    }
}

fn main() {}
