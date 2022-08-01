pub struct Solution {}

impl Solution {
    pub fn modify_string(s: String) -> String {
        let mut chars = s.chars().collect::<Vec<_>>();

        for i in 0..chars.len() {
            if chars[i] == '?' {
                if let Some(c) = ('a'..='z').find(|c| {
                    (i == 0 || &chars[i - 1] != c) && (i == chars.len() - 1 || &chars[i + 1] != c)
                }) {
                    chars[i] = c;
                }
            }
        }

        chars.into_iter().collect()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_1576() {
        assert_eq!(
            Solution::modify_string("?zs".to_string()),
            "azs".to_string()
        );
        assert_eq!(
            Solution::modify_string("ubv?w".to_string()),
            "ubvaw".to_string()
        );
    }
}
