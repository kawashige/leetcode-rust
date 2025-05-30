pub struct Solution {}

impl Solution {
    pub fn is_valid(word: String) -> bool {
        if word.len() < 3 {
            return false;
        }
        let mut exist = [false; 2];
        for c in word.chars() {
            if !c.is_alphanumeric() {
                return false;
            }
            if c.is_alphabetic() {
                exist[if ['a', 'e', 'i', 'o', 'u'].contains(&c.to_ascii_lowercase()) {
                    0
                } else {
                    1
                }] = true;
            }
        }
        exist[0] && exist[1]
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_3136() {
        assert!(Solution::is_valid("234Adas".to_string()));
        assert!(!Solution::is_valid("b3".to_string()));
        assert!(!Solution::is_valid("a3$e".to_string()));
    }
}
