pub struct Solution {}

impl Solution {
    pub fn number_of_ways(s: String) -> i64 {
        let mut dp = vec![vec![0; 4]; 2];
        dp[0][0] = 1;
        dp[1][0] = 1;

        for i in 0..s.len() {
            let current_t = (s.as_bytes()[i] - b'0') as usize;
            let prev_t = (current_t + 1) % 2;
            for j in 0..3 {
                dp[current_t][j + 1] += dp[prev_t][j];
            }
        }

        dp[0][3] + dp[1][3]
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_2222() {
        assert_eq!(Solution::number_of_ways("001101".to_string()), 6);
        assert_eq!(Solution::number_of_ways("11100".to_string()), 0);
    }
}
