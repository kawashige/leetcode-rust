pub struct Solution {}

impl Solution {
    pub fn reorder_spaces(text: String) -> String {
        let spaces = text.as_bytes().iter().filter(|b| b == &&b' ').count();
        let words = text
            .split_ascii_whitespace()
            .map(|s| s.to_string())
            .collect::<Vec<_>>();
        if words.len() == 1 {
            return format!(
                "{}{}",
                words[0],
                std::iter::repeat(' ').take(spaces).collect::<String>()
            );
        }

        let result = words.join(
            std::iter::repeat(' ')
                .take(spaces / (words.len() - 1))
                .collect::<String>()
                .as_str(),
        );
        format!(
            "{}{}",
            result,
            std::iter::repeat(' ')
                .take(spaces % (words.len() - 1))
                .collect::<String>()
        )
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_1592() {
        assert_eq!(Solution::reorder_spaces("a".to_string()), "a".to_string());
        assert_eq!(
            Solution::reorder_spaces("  this   is  a sentence ".to_string()),
            "this   is   a   sentence".to_string()
        );
        assert_eq!(
            Solution::reorder_spaces(" practice   makes   perfect".to_string()),
            "practice   makes   perfect ".to_string()
        );
    }
}
