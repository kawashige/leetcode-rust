pub struct Solution {}

impl Solution {
    pub fn longest_continuous_substring(s: String) -> i32 {
        let mut result = 1;
        let mut lenght = 0;

        for i in 0..s.len() {
            if 0 < i && s.as_bytes()[i] as i32 - s.as_bytes()[i - 1] as i32 == 1 {
                lenght += 1;
                result = result.max(lenght);
            } else {
                lenght = 1;
            }
        }

        result
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_2414() {
        assert_eq!(
            Solution::longest_continuous_substring("abacaba".to_string()),
            2
        );
        assert_eq!(
            Solution::longest_continuous_substring("abcde".to_string()),
            5
        );
    }
}
