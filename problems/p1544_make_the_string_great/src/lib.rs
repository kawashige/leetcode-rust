pub struct Solution {}

impl Solution {
    pub fn make_good(s: String) -> String {
        let mut stack: Vec<char> = Vec::with_capacity(s.len());
        for c in s.chars() {
            if let Some(top) = stack.last() {
                if top != &c && top.to_ascii_lowercase() == c.to_ascii_lowercase() {
                    stack.pop();
                } else {
                    stack.push(c);
                }
            } else {
                stack.push(c);
            }
        }
        stack.into_iter().collect()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_1544() {
        assert_eq!(
            Solution::make_good("leEeetcode".to_string()),
            "leetcode".to_string()
        );
        assert_eq!(Solution::make_good("abBAcC".to_string()), "".to_string());
        assert_eq!(Solution::make_good("s".to_string()), "s".to_string());
    }
}
