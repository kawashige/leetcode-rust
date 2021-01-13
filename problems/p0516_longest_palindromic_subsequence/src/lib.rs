pub struct Solution {}

impl Solution {
    pub fn longest_palindrome_subseq(s: String) -> i32 {
        let s_chars = s.chars().collect::<Vec<char>>();
        let mut dp = vec![vec![0; s_chars.len()]; s_chars.len()];

        for i in (0..s_chars.len()).rev() {
            dp[i][i] = 1;
            for j in (i + 1)..s_chars.len() {
                if s_chars[i] == s_chars[j] {
                    dp[i][j] = dp[i + 1][j - 1] + 2;
                } else {
                    dp[i][j] = std::cmp::max(dp[i + 1][j], dp[i][j - 1]);
                }
            }
        }
        dp[0][s_chars.len() - 1]
    }
}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn test_0516() {
        assert_eq!(4, Solution::longest_palindrome_subseq("bbbab".to_string()));
        assert_eq!(2, Solution::longest_palindrome_subseq("cbbd".to_string()));
        assert_eq!(0, Solution::longest_palindrome_subseq("".to_string()));
        assert_eq!(1, Solution::longest_palindrome_subseq("a".to_string()));
        assert_eq!(
            13,
            Solution::longest_palindrome_subseq("abcdefgfedcba".to_string())
        );
    }
}
