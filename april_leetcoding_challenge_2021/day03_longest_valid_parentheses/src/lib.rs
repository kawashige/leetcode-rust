pub struct Solution {}

impl Solution {
    pub fn longest_valid_parentheses(s: String) -> i32 {
        if s.is_empty() {
            return 0;
        }

        let chars = s.chars().collect::<Vec<_>>();
        let mut dp = vec![vec![false; s.len()]; s.len()];
        let mut dp2 = vec![0; s.len()];

        for i in 0..s.len() {
            if chars[i] == '(' {
                continue;
            }
            let mut tmp = 0;
            for j in 0..i {
                if chars[j] == '(' && (j + 1 == i || dp[j + 1][i - 1]) {
                    dp[j][i] = true;
                    if j > 0 && dp2[j - 1] > 0 {
                        dp[j - dp2[j - 1]][i] = true;
                    }
                    tmp = std::cmp::max(tmp, i - j + 1 + if j > 0 { dp2[j - 1] } else { 0 });
                }
            }
            dp2[i] = tmp;
        }

        dp2.into_iter().max().unwrap() as i32
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_day03() {
        assert_eq!(Solution::longest_valid_parentheses("(()())".to_string()), 6);
        assert_eq!(
            Solution::longest_valid_parentheses("((()))())".to_string()),
            8
        );
        assert_eq!(Solution::longest_valid_parentheses("()(()".to_string()), 2);
        assert_eq!(Solution::longest_valid_parentheses("()(())".to_string()), 6);
        assert_eq!(Solution::longest_valid_parentheses("(()".to_string()), 2);
        assert_eq!(Solution::longest_valid_parentheses(")()())".to_string()), 4);
        assert_eq!(Solution::longest_valid_parentheses("".to_string()), 0);
    }
}
