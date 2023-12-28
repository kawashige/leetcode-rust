pub struct Solution {}

impl Solution {
    pub fn greatest_letter(s: String) -> String {
        ('A'..='Z')
            .rev()
            .find(|c| s.contains(*c) && s.contains(c.to_ascii_lowercase()))
            .unwrap_or(' ')
            .to_string()
            .trim()
            .to_string()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_2309() {
        assert_eq!(
            Solution::greatest_letter("lEeTcOdE".to_string()),
            "E".to_string()
        );
        assert_eq!(
            Solution::greatest_letter("arRAzFif".to_string()),
            "R".to_string()
        );
        assert_eq!(
            Solution::greatest_letter("AbCdEfGhIjK".to_string()),
            "".to_string()
        );
    }
}
