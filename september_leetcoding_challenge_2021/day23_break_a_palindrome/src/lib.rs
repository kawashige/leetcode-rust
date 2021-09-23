pub struct Solution {}

impl Solution {
    pub fn break_palindrome(palindrome: String) -> String {
        if palindrome.len() == 1 {
            return Default::default();
        }

        let mut chars = palindrome.chars().collect::<Vec<_>>();

        if let Some(i) = (0..chars.len()).find(|i| chars[*i] != 'a') {
            if ((i + 1)..chars.len()).any(|j| chars[j] != 'a') {
                chars[i] = 'a';
            } else {
                *chars.last_mut().unwrap() = 'b';
            }
        } else {
            *chars.last_mut().unwrap() = 'b';
        }

        chars.into_iter().collect()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_day23() {
        assert_eq!(
            Solution::break_palindrome("abccba".to_string()),
            "aaccba".to_string()
        );
        assert_eq!(Solution::break_palindrome("a".to_string()), "".to_string());
        assert_eq!(
            Solution::break_palindrome("aba".to_string()),
            "abb".to_string()
        );
    }
}
