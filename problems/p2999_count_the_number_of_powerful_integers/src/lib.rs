pub struct Solution {}

impl Solution {
    pub fn count(n: i64, limit: i32, s: &str) -> i64 {
        if n == 1 {
            return 0;
        }
        let s_value = s.parse::<i64>().unwrap();
        if s_value == n {
            return 1;
        } else if n < s_value {
            return 0;
        }

        let n_s = n.to_string();
        let mut dp = vec![vec![0; 2]; n_s.len() + 1];
        dp[0][0] = 1;
        for i in 1..dp.len() {
            if n_s.len() - s.len() < i {
                if n_s.as_bytes()[i - 1] == s.as_bytes()[i - (n_s.len() - s.len()) - 1] {
                    dp[i][0] = dp[i - 1][0];
                }
                if n_s.as_bytes()[i - 1] > s.as_bytes()[i - (n_s.len() - s.len()) - 1] {
                    dp[i][1] = dp[i - 1][0];
                }
                dp[i][1] += dp[i - 1][1];
            } else {
                if (n_s.as_bytes()[i - 1] - b'0') as i32 <= limit {
                    dp[i][0] = dp[i - 1][0];
                }
                dp[i][1] =
                    dp[i - 1][0] * (((n_s.as_bytes()[i - 1] - b'0') as i64).min(limit as i64 + 1));
                dp[i][1] += dp[i - 1][1] * (limit as i64 + 1);
            }
        }

        dp.last().unwrap().iter().sum()
    }

    pub fn number_of_powerful_int(start: i64, finish: i64, limit: i32, s: String) -> i64 {
        Self::count(finish, limit, &s) - Self::count(start - 1, limit, &s)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_2999() {
        assert_eq!(
            Solution::number_of_powerful_int(10, 1844, 5, "12".to_string()),
            12
        );
        assert_eq!(
            Solution::number_of_powerful_int(1, 6000, 4, "124".to_string()),
            5
        );
        assert_eq!(
            Solution::number_of_powerful_int(15, 215, 6, "10".to_string()),
            2
        );
        assert_eq!(
            Solution::number_of_powerful_int(1000, 2000, 4, "3000".to_string()),
            0
        );
    }
}
