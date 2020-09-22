pub struct Solution {}

impl Solution {
    pub fn reverse_words(s: String) -> String {
        s.split_ascii_whitespace()
            .rev()
            .collect::<Vec<&str>>()
            .join(" ")
    }
}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn test_0151() {
        assert_eq!(
            "blue is sky the".to_string(),
            Solution::reverse_words("the sky is blue".to_string())
        );
        assert_eq!(
            "world! hello".to_string(),
            Solution::reverse_words("  hello world!  ".to_string())
        );
        assert_eq!(
            "example good a".to_string(),
            Solution::reverse_words("a good    example".to_string())
        );
    }
}
