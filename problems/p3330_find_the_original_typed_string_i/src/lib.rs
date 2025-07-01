pub struct Solution {}

impl Solution {
    pub fn possible_string_count(word: String) -> i32 {
        let mut result = 1;
        for i in 1..word.len() {
            if word.as_bytes()[i - 1] == word.as_bytes()[i] {
                result += 1;
            }
        }
        result
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_3330() {
        assert_eq!(Solution::possible_string_count("abbcccc".to_string()), 5);
        assert_eq!(Solution::possible_string_count("abcd".to_string()), 1);
        assert_eq!(Solution::possible_string_count("aaaa".to_string()), 4);
    }
}
