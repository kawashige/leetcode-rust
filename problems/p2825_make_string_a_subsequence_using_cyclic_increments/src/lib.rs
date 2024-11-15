pub struct Solution {}

impl Solution {
    pub fn can_make_subsequence(str1: String, str2: String) -> bool {
        let mut i = 0;
        for j in 0..str1.len() {
            if str1.as_bytes()[j] == str2.as_bytes()[i]
                || ((str1.as_bytes()[j] - b'a' + 27) % 26 == str2.as_bytes()[i] - b'a')
            {
                i += 1;
                if i == str2.len() {
                    return true;
                }
            }
        }
        false
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_2825() {
        assert!(Solution::can_make_subsequence(
            "abc".to_string(),
            "ad".to_string()
        ));
        assert!(Solution::can_make_subsequence(
            "zc".to_string(),
            "ad".to_string()
        ));
        assert!(!Solution::can_make_subsequence(
            "ab".to_string(),
            "d".to_string()
        ));
    }
}
