pub struct Solution {}

impl Solution {
    pub fn count_valid_words(sentence: String) -> i32 {
        sentence
            .split_whitespace()
            .filter(|s| {
                let chars = s.chars().collect::<Vec<_>>();
                let mut hyphen = 0;
                for i in 0..chars.len() {
                    if chars[i].is_digit(10) {
                        return false;
                    }
                    if chars[i] == '-' {
                        if hyphen != 0
                            || !(1..chars.len() - 1).contains(&i)
                            || (!chars[i - 1].is_ascii_lowercase()
                                || !chars[i + 1].is_ascii_lowercase())
                        {
                            return false;
                        }
                        hyphen += 1;
                    }
                    if ['!', '.', ','].contains(&chars[i]) && i != chars.len() - 1 {
                        return false;
                    }
                }
                true
            })
            .count() as i32
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_2047() {
        assert_eq!(Solution::count_valid_words("cat and  dog".to_string()), 3);
        assert_eq!(
            Solution::count_valid_words("!this  1-s b8d!".to_string()),
            0
        );
        assert_eq!(
            Solution::count_valid_words("alice and  bob are playing stone-game10".to_string()),
            5
        );
    }
}
