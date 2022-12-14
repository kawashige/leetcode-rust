pub struct Solution {}

impl Solution {
    pub fn truncate_sentence(s: String, k: i32) -> String {
        s.split_whitespace()
            .take(k as usize)
            .collect::<Vec<_>>()
            .join(" ")
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_1816() {
        assert_eq!(
            Solution::truncate_sentence("Hello how are you Contestant".to_string(), 4),
            "Hello how are you".to_string()
        );
        assert_eq!(
            Solution::truncate_sentence("What is the solution to this problem".to_string(), 4),
            "What is the solution".to_string()
        );
        assert_eq!(
            Solution::truncate_sentence("chopper is not a tanuki".to_string(), 5),
            "chopper is not a tanuki".to_string()
        );
    }
}
