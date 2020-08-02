pub struct Solution {}

impl Solution {
    pub fn length_of_last_word(s: String) -> i32 {
        match s.trim().rfind(' ') {
            Some(i) => (s.trim().len() - 1 - i) as i32,
            None => s.trim().len() as i32,
        }
    }
}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn test_58() {
        assert_eq!(1, Solution::length_of_last_word("a ".to_string()));
    }
}
