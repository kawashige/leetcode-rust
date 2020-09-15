pub struct Solution {}

impl Solution {
    pub fn length_of_last_word(s: String) -> i32 {
        s.trim_end().split(" ").last().unwrap_or("").len() as i32
    }
}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn test_day15() {
        assert_eq!(5, Solution::length_of_last_word("Hello World".to_string()));
        assert_eq!(0, Solution::length_of_last_word("".to_string()));
        assert_eq!(1, Solution::length_of_last_word("a ".to_string()));
        assert_eq!(1, Solution::length_of_last_word(" a".to_string()));
        assert_eq!(0, Solution::length_of_last_word("  ".to_string()));
    }
}
