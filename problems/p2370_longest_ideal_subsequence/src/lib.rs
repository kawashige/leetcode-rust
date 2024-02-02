pub struct Solution {}

impl Solution {
    pub fn longest_ideal_string(s: String, k: i32) -> i32 {
        let mut dp = vec![vec![0; 26]; s.len() + 1];

        for i in 0..s.len() {
            let order = (s.as_bytes()[i] - b'a') as usize;
            for j in 0..26 {
                if (order as i32 - k..=order as i32 + k).contains(&j) {
                    dp[i + 1][order] = dp[i + 1][order].max(dp[i][j as usize] + 1);
                }
                dp[i + 1][j as usize] = dp[i + 1][j as usize].max(dp[i][j as usize]);
            }
        }
        *dp.last().unwrap().into_iter().max().unwrap()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_2370() {
        assert_eq!(Solution::longest_ideal_string("acfgbd".to_string(), 2), 4);
        assert_eq!(Solution::longest_ideal_string("abcd".to_string(), 3), 4);
    }
}
