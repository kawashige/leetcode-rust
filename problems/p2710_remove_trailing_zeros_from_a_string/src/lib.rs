pub struct Solution {}

impl Solution {
    pub fn remove_trailing_zeros(num: String) -> String {
        num.trim_end_matches('0').to_string()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_2710() {
        assert_eq!(
            Solution::remove_trailing_zeros("51230100".to_string()),
            "512301".to_string()
        );
        assert_eq!(
            Solution::remove_trailing_zeros("123".to_string()),
            "123".to_string()
        );
    }
}
