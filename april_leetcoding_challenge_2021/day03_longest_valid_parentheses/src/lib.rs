pub struct Solution {}

impl Solution {
    pub fn longest_valid_parentheses(s: String) -> i32 {
        if s.is_empty() {
            return 0;
        }

        let chars = s.chars().collect::<Vec<_>>();
        let mut dp = vec![0; s.len()];

        let mut max = 0;
        for i in 1..s.len() {
            if chars[i] == '(' {
                continue;
            }
            if chars[i - 1] == '(' {
                dp[i] = if i >= 2 { dp[i - 2] } else { 0 } + 2;
            } else if i - dp[i - 1] > 0 && chars[i - dp[i - 1] - 1] == '(' {
                dp[i] = dp[i - 1]
                    + if i - dp[i - 1] >= 2 {
                        dp[i - dp[i - 1] - 2]
                    } else {
                        0
                    }
                    + 2;
            }
            max = std::cmp::max(max, dp[i]);
        }

        max as i32
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
