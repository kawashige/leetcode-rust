pub struct Solution {}

impl Solution {
    pub fn is_palindrome(bytes: &[u8], i: usize) -> bool {
        for j in i..bytes.len() / 2 {
            if bytes[j] != bytes[bytes.len() - 1 - j] {
                return false;
            }
        }
        true
    }

    pub fn check_palindrome_formation(a: String, b: String) -> bool {
        let l = a.len();
        let bytes_a = a.as_bytes();
        let bytes_b = b.as_bytes();

        let mut i = 0;
        while i < l / 2 && bytes_a[i] == bytes_b[l - 1 - i] {
            i += 1;
        }

        if Self::is_palindrome(bytes_a, i) || Self::is_palindrome(bytes_b, i) {
            return true;
        }

        let mut i = 0;
        while i < l / 2 && bytes_b[i] == bytes_a[l - 1 - i] {
            i += 1;
        }
        Self::is_palindrome(bytes_a, i) || Self::is_palindrome(bytes_b, i)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_1616() {
        assert!(Solution::check_palindrome_formation(
            "pvhmupgqeltozftlmfjjde".to_string(),
            "yjgpzbezspnnpszebzmhvp".to_string()
        ));
        assert!(!Solution::check_palindrome_formation(
            "abda".to_string(),
            "acmc".to_string()
        ));
        assert!(Solution::check_palindrome_formation(
            "x".to_string(),
            "y".to_string()
        ));
        assert!(!Solution::check_palindrome_formation(
            "xbdef".to_string(),
            "xecab".to_string()
        ));
        assert!(Solution::check_palindrome_formation(
            "ulacfd".to_string(),
            "jizalu".to_string()
        ));
    }
}
