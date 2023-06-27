pub struct Solution {}

impl Solution {
    pub fn are_numbers_ascending(s: String) -> bool {
        s.split_ascii_whitespace()
            .filter(|i| !i.starts_with(char::is_alphabetic))
            .collect::<Vec<_>>()
            .windows(2)
            .all(|v| v[0].parse::<usize>().unwrap() < v[1].parse::<usize>().unwrap())
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_2042() {
        assert!(Solution::are_numbers_ascending(
            "1 box has 3 blue 4 red 6 green and 12 yellow marbles".to_string()
        ));
        assert!(!Solution::are_numbers_ascending(
            "hello world 5 x 5".to_string()
        ));
        assert!(!Solution::are_numbers_ascending(
            "sunset is at 7 51 pm overnight lows will be in the low 50 and 60 s".to_string()
        ));
    }
}
