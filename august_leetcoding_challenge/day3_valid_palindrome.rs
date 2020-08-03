pub struct Solution {}

impl Solution {
    pub fn is_palindrome(s: String) -> bool {
        let chars = s.chars().collect::<Vec<char>>();
        if chars.len() == 0 {
            return true;
        }

        let mut i = 0;
        let mut j = chars.len() - 1;
        while i < chars.len() && !chars[i].is_ascii_alphanumeric() {
            i += 1;
        }
        while j > 0 && !chars[j].is_ascii_alphanumeric() {
            j -= 1;
        }
        while i < j {
            if !chars[i].eq_ignore_ascii_case(&chars[j]) {
                return false;
            }
            i += 1;
            j -= 1;
            while i < chars.len() && !chars[i].is_ascii_alphanumeric() {
                i += 1;
            }
            while j > 0 && !chars[j].is_ascii_alphanumeric() {
                j -= 1;
            }
        }
        true
    }
}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn test_day3() {
        assert_eq!(
            true,
            Solution::is_palindrome("A man, a plan, a canal: Panama".to_string())
        );
        assert_eq!(false, Solution::is_palindrome("race a car".to_string()));
        assert_eq!(true, Solution::is_palindrome("".to_string()));
        assert_eq!(true, Solution::is_palindrome("a".to_string()));
        assert_eq!(false, Solution::is_palindrome("0P".to_string()));
    }
}
