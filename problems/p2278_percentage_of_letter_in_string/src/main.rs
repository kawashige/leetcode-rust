pub struct Solution {}

impl Solution {
    pub fn percentage_letter(s: String, letter: char) -> i32 {
        (s.chars().filter(|c| c == &letter).count() * 100 / s.len()) as i32
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_2278() {
        assert_eq!(Solution::percentage_letter("foobar".to_string(), 'o'), 33);
        assert_eq!(Solution::percentage_letter("jjjj".to_string(), 'k'), 0);
    }
}
