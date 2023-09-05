pub struct Solution {}

impl Solution {
    pub fn capitalize_title(title: String) -> String {
        title
            .split_whitespace()
            .map(|w| {
                if w.len() < 3 {
                    w.to_lowercase()
                } else {
                    w.chars()
                        .enumerate()
                        .map(|(i, c)| {
                            if i == 0 {
                                c.to_ascii_uppercase()
                            } else {
                                c.to_ascii_lowercase()
                            }
                        })
                        .collect()
                }
            })
            .collect::<Vec<_>>()
            .join(" ")
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_2129() {
        assert_eq!(
            Solution::capitalize_title("capiTalIze tHe titLe".to_string()),
            "Capitalize The Title".to_string()
        );
        assert_eq!(
            Solution::capitalize_title("First leTTeR of EACH Word".to_string()),
            "First Letter of Each Word".to_string()
        );
        assert_eq!(
            Solution::capitalize_title("i lOve leetcode".to_string()),
            "i Love Leetcode".to_string()
        );
    }
}
