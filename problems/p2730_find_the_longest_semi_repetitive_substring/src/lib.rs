pub struct Solution {}

impl Solution {
    pub fn longest_semi_repetitive_substring(s: String) -> i32 {
        let mut result = 0;
        for i in 0..s.len() {
            let mut count = 0;
            let mut len = 1;
            for j in (0..i).rev() {
                if s.as_bytes()[j] == s.as_bytes()[j + 1] {
                    count += 1;
                    if count == 2 {
                        break;
                    }
                }
                len += 1;
            }
            result = result.max(len);
        }
        result
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_2730() {
        assert_eq!(
            Solution::longest_semi_repetitive_substring("52233".to_string()),
            4
        );
        assert_eq!(
            Solution::longest_semi_repetitive_substring("5494".to_string()),
            4
        );
        assert_eq!(
            Solution::longest_semi_repetitive_substring("1111111".to_string()),
            2
        );
    }
}
