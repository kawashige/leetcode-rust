pub struct Solution {}

impl Solution {
    pub fn find_ocurrences(text: String, first: String, second: String) -> Vec<String> {
        text.split_ascii_whitespace()
            .collect::<Vec<_>>()
            .windows(3)
            .filter(|strings| strings[0] == first && strings[1] == second)
            .map(|strings| strings[2].to_string())
            .collect()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_1078() {
        assert_eq!(
            Solution::find_ocurrences(
                "alice is a good girl she is a good student".to_string(),
                "a".to_string(),
                "good".to_string()
            ),
            vec!["girl".to_string(), "student".to_string()]
        );

        assert_eq!(
            Solution::find_ocurrences(
                "we will we will rock you".to_string(),
                "we".to_string(),
                "will".to_string()
            ),
            vec!["we".to_string(), "rock".to_string()]
        );
    }
}
