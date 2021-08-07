pub struct Solution {}

impl Solution {
    pub fn min_cut(s: String) -> i32 {
        let n = s.len();
        let bytes = s.as_bytes();
        let mut is_palindrome = vec![vec![false; n]; n];

        for l in 0..n {
            for i in 0..(n - l) {
                is_palindrome[i][i + l] = match l {
                    0 => true,
                    1 => bytes[i] == bytes[i + 1],
                    _ => bytes[i] == bytes[i + l] && is_palindrome[i + 1][i + l - 1],
                };
            }
        }

        let mut dp = (0..n).collect::<Vec<_>>();

        for i in 0..n {
            for j in 0..=i {
                if is_palindrome[j][i] {
                    dp[i] = dp[i].min(if j == 0 { 0 } else { dp[j - 1] + 1 });
                }
            }
        }

        dp[n - 1] as i32
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_day07() {
        assert_eq!(Solution::min_cut("aaaaaaabcdedcbaaaaaaa".to_string()), 0);
        assert_eq!(Solution::min_cut("aab".to_string()), 1);
        assert_eq!(Solution::min_cut("a".to_string()), 0);
        assert_eq!(Solution::min_cut("ab".to_string()), 1);
    }
}
