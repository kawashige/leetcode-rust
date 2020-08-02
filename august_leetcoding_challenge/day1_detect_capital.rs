pub struct Solution {}

impl Solution {
    pub fn detect_capital_use(word: String) -> bool {
        let chars = word.chars().collect::<Vec<char>>();
        chars.iter().all(|c| c.is_uppercase())
            || chars.iter().all(|c| c.is_lowercase())
            || chars[0].is_uppercase() && chars[1..].iter().all(|c| c.is_lowercase())
    }
}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn test_day1() {
        assert_eq!(true, Solution::detect_capital_use("USA".to_string()));
        assert_eq!(true, Solution::detect_capital_use("leetcode".to_string()));
        assert_eq!(true, Solution::detect_capital_use("Google".to_string()));
        assert_eq!(false, Solution::detect_capital_use("FlaG".to_string()));
    }
}
