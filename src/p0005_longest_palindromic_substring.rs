pub struct Solution {}

impl Solution {
    pub fn longest_palindrome(s: String) -> String {
        fn is_palindrome(chars: &[char]) -> bool {
            let mut i = 0 as usize;
            let length = chars.len();
            while i < length - i {
                if chars[i] != chars[length - 1 - i] {
                    return false;
                }
                i += 1;
            }
            true
        }

        let chars = s.chars().collect::<Vec<char>>();
        for l in 0..chars.len() {
            for i in 0..(l + 1) {
                let len = s.len() - l;
                if is_palindrome(&chars[i..(i + len)]) {
                    return chars[i..(i + len)].into_iter().collect::<String>();
                }
            }
        }

        s[0..s.len()].to_string()
    }
}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn test_0005() {
        assert_eq!("bab", Solution::longest_palindrome("babad".to_string()));
        assert_eq!("bb", Solution::longest_palindrome("cbbd".to_string()));
        assert_eq!("", Solution::longest_palindrome("".to_string()));
    }
}
