pub struct Solution {}

impl Solution {
    pub fn str_str(haystack: String, needle: String) -> i32 {
        match &haystack.find(&needle) {
            Some(x) => *x as i32,
            None => -1,
        }
    }
}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn test_28() {
        assert_eq!(2, Solution::str_str("Hello".to_string(), "ll".to_string()));
    }
}
