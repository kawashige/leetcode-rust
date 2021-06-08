pub struct Solution {}

impl Solution {
    pub fn apply_backspace(s: String) -> String {
        s.chars().fold(String::new(), |mut s, c| {
            if c == '#' && !s.is_empty() {
                s.pop();
            } else if c != '#' {
                s.push(c)
            }
            s
        })
    }

    pub fn backspace_compare(s: String, t: String) -> bool {
        Self::apply_backspace(s) == Self::apply_backspace(t)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_0844() {
        assert!(Solution::backspace_compare(
            "ab#c".to_string(),
            "ad#c".to_string()
        ));
        assert!(Solution::backspace_compare(
            "ab##".to_string(),
            "c#d#".to_string()
        ));
        assert!(Solution::backspace_compare(
            "a##c".to_string(),
            "#a#c".to_string()
        ));
        assert!(!Solution::backspace_compare(
            "a#c".to_string(),
            "b".to_string()
        ));
    }
}
