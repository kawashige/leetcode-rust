pub struct Solution {}

impl Solution {
    pub fn check_string(s: String) -> bool {
        println!(
            "{}",
            s.len() - 1 - s.chars().rev().position(|c| c == 'a').unwrap_or(0)
        );
        println!("{}", s.chars().position(|c| c == 'b').unwrap_or(s.len()));
        s.len()
            - 1
            - s.chars()
                .rev()
                .position(|c| c == 'a')
                .unwrap_or(s.len() - 1)
            <= s.chars().position(|c| c == 'b').unwrap_or(s.len())
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_2124() {
        assert!(Solution::check_string("aaabbb".to_string()));
        assert!(!Solution::check_string("abab".to_string()));
        assert!(Solution::check_string("bbb".to_string()));
    }
}
