pub struct Solution {}

impl Solution {
    pub fn generate_tag(caption: String) -> String {
        let s = "#".to_string()
            + &caption
                .split_ascii_whitespace()
                .enumerate()
                .map(|(i, s)| {
                    format!(
                        "{}{}",
                        if i != 0 {
                            s[..1].to_ascii_uppercase()
                        } else {
                            s[..1].to_ascii_lowercase()
                        },
                        s[1..].to_ascii_lowercase()
                    )
                })
                .collect::<String>();
        if 100 < s.len() {
            s[..100].to_string()
        } else {
            s
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_3582() {
        assert_eq!(
            Solution::generate_tag("Leetcode daily streak achieved".to_string()),
            "#leetcodeDailyStreakAchieved".to_string()
        );
        assert_eq!(
            Solution::generate_tag("can I Go There".to_string()),
            "#canIGoThere".to_string()
        );
        assert_eq!(Solution::generate_tag("hhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhh".to_string()), "#hhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhh".to_string());
    }
}
